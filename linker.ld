ENTRY(_start)

SECTIONS
{
    /* On commence à 1 MiB */
    . = 0x00100000;

    /* L'en-tête Multiboot DOIT être au tout début du binaire, aligné sur 4 octets */
    .multiboot ALIGN(4) :
    {
        KEEP(*(.multiboot))
    }

    /* Le code exécutable, aligné sur 16 octets suffit pour le binaire brut */
    .text ALIGN(16) :
    {
        *(.text*)
    }

    /* Données en lecture seule */
    .rodata ALIGN(16) :
    {
        *(.rodata*)
        *(.data.rel.ro*)
    }

    /* Données initialisées */
    .data ALIGN(16) :
    {
        *(.data*)
        *(.got*)
        *(.got.plt*)
    }

    /* Données non-initialisées (la pile) */
    .bss ALIGN(16) :
    {
        *(COMMON)
        *(.bss*)
    }
}