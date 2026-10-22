/** File-level C API documentation. */

/** Adds two integers with an explicit output pointer. */
int add_into(int left, int right, int *result) {
    *result = left + right;
    return 0;
}

int undocumented(int value) { return value; }

/* Ordinary comments are not documentation. */
struct Entry {
    int value;
};
