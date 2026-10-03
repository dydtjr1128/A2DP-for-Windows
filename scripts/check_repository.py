"""Validate this repository's text files, inline local links, and GitHub forms.

No external URL requests, driver operations, or generated-directory traversal.
Requires Python 3.11+ and scripts/requirements-checks.txt.
"""

from pathlib import Path
import re
import subprocess
import sys
import tomllib
from urllib.parse import unquote, urlsplit

import yaml


ROOT = Path(__file__).resolve().parents[1]
PR_HEADINGS = (
    "요약",
    "변경",
    "관련 이슈 및 완료 조건",
    "검증",
    "알려진 문제",
    "제외 범위",
)


class UniqueKeyLoader(yaml.SafeLoader):
    """Reject duplicate YAML keys instead of silently keeping the final value."""

    def construct_mapping(self, node, deep=False):
        keys = set()
        for key_node, _ in node.value:
            key = self.construct_object(key_node, deep=deep)
            if key in keys:
                raise ValueError(f"duplicate YAML key: {key}")
            keys.add(key)
        return super().construct_mapping(node, deep=deep)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def repository_files():
    result = subprocess.run(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"],
        cwd=ROOT,
        check=True,
        capture_output=True,
    )
    names = result.stdout.decode("utf-8").split("\0")
    return sorted({ROOT / name for name in names if name and (ROOT / name).is_file()})


def check_markdown(path, content):
    fence = None
    for number, line in enumerate(content.splitlines(), 1):
        marker = re.match(r"^\s*(`{3,}|~{3,})(.*)$", line)
        if marker:
            token, rest = marker.groups()
            if fence is None:
                fence = token
            elif token[0] == fence[0] and len(token) >= len(fence) and not rest.strip():
                fence = None
            continue
        if fence:
            continue
        for raw_target in re.findall(r"\[[^\]\n]+\]\(([^)\n]+)\)", line):
            target = raw_target.strip().removeprefix("<").removesuffix(">")
            parsed = urlsplit(target)
            if parsed.scheme or parsed.netloc or not parsed.path:
                continue
            destination = path.parent / unquote(parsed.path)
            require(destination.exists(), f"{path}:{number}: missing link target {target}")
    require(fence is None, f"{path}: unclosed code fence")


def check_issue_form(path, data):
    if path.name == "config.yml":
        require(data.get("blank_issues_enabled") is False, f"{path}: expected issue forms")
        return
    require(data.get("name") and data.get("description"), f"{path}: missing form identity")
    require(isinstance(data.get("body"), list) and data["body"], f"{path}: empty form")
    ids = set()
    for item in data["body"]:
        kind = item.get("type")
        require(
            kind in {"markdown", "input", "textarea", "dropdown", "checkboxes"},
            f"{path}: unknown form item {kind}",
        )
        attributes = item.get("attributes", {})
        if kind == "markdown":
            require(attributes.get("value"), f"{path}: empty markdown item")
            continue
        item_id = item.get("id", "")
        require(re.fullmatch(r"[A-Za-z_][A-Za-z0-9_-]*", item_id), f"{path}: invalid ID")
        require(item_id not in ids, f"{path}: duplicate form ID {item_id}")
        ids.add(item_id)
        require(attributes.get("label"), f"{path}: missing label for {item_id}")
        if kind in {"dropdown", "checkboxes"}:
            require(attributes.get("options"), f"{path}: missing options for {item_id}")
    require("summary" in ids, f"{path}: missing summary field")


def main():
    markdown_count = yaml_count = 0
    for path in repository_files():
        if path.suffix not in {".md", ".yml", ".yaml", ".toml", ".rs", ".py", ".ps1", ".txt"}:
            continue
        raw = path.read_bytes()
        require(not raw.startswith(b"\xef\xbb\xbf"), f"{path}: unexpected UTF-8 BOM")
        require(b"\r" not in raw, f"{path}: expected LF line endings")
        require(raw.endswith(b"\n"), f"{path}: missing final newline")
        content = raw.decode("utf-8")
        for number, line in enumerate(content.splitlines(), 1):
            require(line == line.rstrip(), f"{path}:{number}: trailing whitespace")
        if path.suffix == ".md":
            check_markdown(path, content)
            markdown_count += 1
        elif path.suffix in {".yml", ".yaml"}:
            data = yaml.load(content, Loader=UniqueKeyLoader)
            require(isinstance(data, dict), f"{path}: expected YAML mapping")
            if path.parent.name == "ISSUE_TEMPLATE":
                check_issue_form(path, data)
            yaml_count += 1
        elif path.suffix == ".toml":
            tomllib.loads(content)

    template = (ROOT / ".github/pull_request_template.md").read_text(encoding="utf-8")
    for heading in PR_HEADINGS:
        require(f"## {heading}\n" in template, f"PR template: missing heading {heading}")
    agents = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
    require("미설정" not in agents, "AGENTS.md: unresolved project settings")
    require("@AGENTS.md" in (ROOT / "CLAUDE.md").read_text(encoding="utf-8"), "missing import")
    print(f"PASS: text files, {markdown_count} Markdown files, {yaml_count} YAML files, TOML, templates")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, subprocess.CalledProcessError, yaml.YAMLError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
