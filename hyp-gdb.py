import gdb

# TODO: QEMU doesn't seem to expose all the system registers to GDB!. For now,
# we just assume EL2.
def get_el():
    return 2

def is_block_descriptor(descriptor, level):
    return level == 3 or (0b11 & descriptor) == 0b01

def entry_span(level):
    match level:
        case 0:
            return 0x8000000000
        case 1:
            return 0x40000000
        case 2:
            return 0x200000
        case 3:
            return 0x1000

class TranslationTableWalk(gdb.Command):
    def __init__ (self):
        super (TranslationTableWalk, self).__init__ ("ttwalk", gdb.COMMAND_USER)

    def invoke (self, arg, from_tty):
        inferior = gdb.selected_inferior()
        arch = inferior.architecture()

        if arch.name() != "aarch64":
            print("Unspported arch")
            return

        frame = gdb.selected_frame()
        ttbr0 = frame.read_register(f"TTBR0_EL{get_el()}")

        self._walk_table(ttbr0, 0, 0)

    def _walk_table(self, addr, level, va):
        inferior = gdb.selected_inferior()

        for (index, descriptor_addr) in enumerate(range(addr, addr+0x1000, 8)):
            descriptor = int.from_bytes(inferior.read_memory(descriptor_addr, 8), "little")

            descriptor_span_begin = va + index * entry_span(level)

            if descriptor == 0:
                continue

            if is_block_descriptor(descriptor, level):
                self._print_block_entry(descriptor, descriptor_addr, level, descriptor_span_begin)
                continue


            self._print_table_entry(descriptor, descriptor_addr, level, descriptor_span_begin)
            next_table_addr = descriptor & ((1 << 35) - 1) << 12

            self._walk_table(next_table_addr, level + 1, va + (index * entry_span(level)))

    @staticmethod
    def _print_block_entry(descriptor, addr, level, va):
        print(f"{' ' * 4 * level}block@{hex(addr)}: {hex(va)}-{hex(va+entry_span(level))}")

    @staticmethod
    def _print_table_entry(descriptor, addr, level, va):
        print(f"{' ' * 4 * level}table@{hex(addr)}")

TranslationTableWalk()
