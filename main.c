#include <rust.h>
#include <stdio.h>

int main(void) {
    rust_test();

    Shape* square = rust_shape_new_square(4);

    Future_f32* future = rust_shape_calculate_area(square);

    const float result = rust_await_f32(future);
    printf("area: %f\n", result);

    rust_shape_free(square);
}
