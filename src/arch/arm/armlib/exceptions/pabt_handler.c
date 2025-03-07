/**
 * @file
 * @brief
 *
 * @author Aleksey Zhmulin
 * @date 06.11.22
 */

#include "exception_priv.h"

#include <inttypes.h>

#include <kernel/printk.h>

void arm_pabt_handler(struct pt_regs *pt_regs, uint32_t status) {
	printk("\nUnresolvable prefetch abort exception!\n");
	printk("IFSR = %#08" PRIx32 "\n", status);
	PRINT_PTREGS(pt_regs);

#if KEEP_GOING
	while (1)
		;
#endif
}
