"""# Python adapter example

Python module, class, and function docstrings become Markdown documentation.
"""

MAX_ITEMS = 25


class Catalog:
    """A small collection with a documented lookup method."""

    def find(self, item_id: str) -> str | None:
        """Find an item by its stable identifier.

        Returns `None` when the item is not present.
        """
        return item_id


def normalize_title(value: str) -> str:
    """Normalize a title by trimming surrounding whitespace."""
    return value.strip()
