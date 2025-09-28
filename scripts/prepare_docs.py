import os
import shutil


def copy_rules_resources_to_docs():
    """Copy rules resources to docs directory for documentation purposes."""
    source_dir = "crates/unsoundness_checker/resources/rules"
    target_dir = "docs/rules"

    os.mkdir(target_dir)

    shutil.copytree(source_dir, target_dir, dirs_exist_ok=True)
    print(f"Successfully copied rules from {source_dir} to {target_dir}")


def main() -> None:
    copy_rules_resources_to_docs()


if __name__ == "__main__":
    main()
