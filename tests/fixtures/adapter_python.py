"""Python module documentation."""

MAX_SIZE = 20
ordinary_value = 3

class Store:
    """A documented store."""

    def find(self, keys: list[tuple[int, str]], *, limit: int = 10) -> tuple[str, ...]:
        """Find values for the requested keys."""
        return ()

    def undocumented(self, value: int) -> int:
        return value

def normalize(value: str) -> str:
    """Normalize a value."""
    return value.strip()
