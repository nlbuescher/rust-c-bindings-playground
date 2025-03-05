#include <rust.h>
#include <stdio.h>

int main(void) {
    rust_test();

    Shape* square = rust_shape_new_square(4);

    const uint32_t area = rust_shape_area(square);
    printf("area: %u\n", area);

    rust_shape_free(square);

    // second access causes segmentation fault, as it should
    //const uint32_t area2 = rust_shape_area(square);
    //printf("area: %u\n", area2);
}
