/**
 * @brief
 *
 * @date 6.07.2011
 * @author Alexandr Kalmuk
 */

#include <stdio.h>
#include <embox/test.h>
#include <mem/objalloc.h>

#define TOTAL_OBJECTS 16

EMBOX_TEST_SUITE("objalloc test");

OBJALLOC_DEF(allocator, int , TOTAL_OBJECTS);

TEST_CASE("test for macro OBJALLOC_DEF") {
	void *obj = objalloc(&allocator);
	test_assert_not_null(obj);
	test_assert_equal(objfree(&allocator,obj, 0);
}
