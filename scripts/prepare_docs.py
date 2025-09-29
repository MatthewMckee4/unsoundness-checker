import glob
import os
import re
import shutil
from typing import Dict


def copy_rules_resources_to_docs():
    """Copy rules resources to docs directory for documentation purposes."""
    source_dir = "crates/unsoundness_checker/resources/rules"
    target_dir = "docs/rules"

    os.makedirs(target_dir, exist_ok=True)

    shutil.copytree(source_dir, target_dir, dirs_exist_ok=True)
    print(f"Successfully copied rules from {source_dir} to {target_dir}")


def read_snapshot_file(filepath: str) -> str:
    """Read a snapshot file and remove the header section."""
    with open(filepath, "r") as f:
        content = f.read()

    lines = content.split("\n")
    header_end = -1
    dash_count = 0

    for i, line in enumerate(lines):
        if line.strip() == "---":
            dash_count += 1
            if dash_count == 2:
                header_end = i
                break

    if header_end >= 0:
        diagnostic_content = "\n".join(lines[header_end + 1 :]).strip()
        return diagnostic_content

    return content.strip()


def get_snapshots_for_rule(rule_name: str) -> Dict[str, str]:
    """Get all snapshot files for a given rule and return their content."""
    snapshots_dir = f"crates/unsoundness_checker/tests/snapshots/{rule_name}"

    if not os.path.exists(snapshots_dir):
        print(f"Warning: No snapshots directory found for rule {rule_name}")
        return {}

    snapshot_files = glob.glob(os.path.join(snapshots_dir, "*.snap"))
    snapshots = {}

    for filepath in sorted(snapshot_files):
        filename = os.path.basename(filepath)

        match = re.search(r"snippet_(\d+)", filename)
        if match:
            snippet_num = match.group(1)
            diagnostic = read_snapshot_file(filepath)

            snapshots[snippet_num] = diagnostic

    return snapshots


def embed_diagnostics_in_markdown(markdown_path: str, rule_name: str):
    """Embed diagnostic outputs into markdown files using collapsible sections."""
    snapshots = get_snapshots_for_rule(rule_name)

    if not snapshots:
        print(f"No diagnostics found for rule {rule_name}")
        return

    with open(markdown_path, "r") as f:
        content = f.read()

    lines = content.split("\n")
    new_lines = []
    code_block_count = 0
    in_code_block = False

    i = 0
    while i < len(lines):
        line = lines[i]
        new_lines.append(line)

        if line.strip().startswith("```py"):
            in_code_block = True

        elif line.strip() == "```" and in_code_block:
            in_code_block = False
            code_block_count += 1

            snippet_key = f"{code_block_count:02d}"
            if snippet_key in snapshots:
                diagnostic = snapshots[snippet_key]

                new_lines.append("")
                if diagnostic.strip():
                    new_lines.append('??? note "Diagnostic Output"')
                    new_lines.append("    ```")
                    for diag_line in diagnostic.split("\n"):
                        new_lines.append(f"    {diag_line}")
                    new_lines.append("    ```")
                else:
                    new_lines.append('!!! info "No Diagnostic Emitted"')

                new_lines.append("")

        i += 1

    with open(markdown_path, "w") as f:
        f.write("\n".join(new_lines))

    print(f"Embedded {len(snapshots)} diagnostics in {markdown_path}")


def update_markdown_files_with_diagnostics():
    """Update all markdown files in docs/rules with diagnostic outputs."""
    rules_dir = "docs/rules"

    if not os.path.exists(rules_dir):
        print(f"Rules directory {rules_dir} does not exist")
        return

    for markdown_file in glob.glob(os.path.join(rules_dir, "*.md")):
        filename = os.path.basename(markdown_file)
        rule_name = filename.replace(".md", "")

        print(f"Processing {filename} for rule {rule_name}")
        embed_diagnostics_in_markdown(markdown_file, rule_name)


def main() -> None:
    copy_rules_resources_to_docs()
    update_markdown_files_with_diagnostics()


if __name__ == "__main__":
    main()
