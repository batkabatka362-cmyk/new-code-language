// ==============================================================================
// CRON High-Performance PCIe Gen5 AXI4 DMA Driver Interface
// Target: 256-Core 4D-Torus Hardware Accelerator (OpenLane SkyWater 130nm ASIC Tapeout (Open-Source Silicon))
// Zero-Copy Host Memory Streaming & Circular Descriptor Rings
// ==============================================================================

#ifndef CRON_PCIE_DMA_H
#define CRON_PCIE_DMA_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Physical Memory-Mapped CSR Register Offsets
#define CRON_REG_MAGIC          0x0000 // Expected: 0x43524F4E ('CRON')
#define CRON_REG_VERSION        0x0004 // Silicon ABI Version
#define CRON_REG_CORE_COUNT     0x0008 // Total Physical Cores (e.g. 256)
#define CRON_REG_DMA_CTRL       0x0010 // DMA Start, Abort, IRQ Enable
#define CRON_REG_DMA_STATUS     0x0014 // Busy, Done, Error flags
#define CRON_REG_DMA_RING_BASE  0x0018 // 64-bit Descriptor Ring Physical Address
#define CRON_REG_DMA_RING_SIZE  0x0020 // Descriptor Count
#define CRON_REG_DMA_HEAD_PTR   0x0024 // Host Submission Doorbell Pointer
#define CRON_REG_DMA_TAIL_PTR   0x0028 // Card Completion Pointer
#define CRON_REG_TORUS_STATUS   0x0030 // 4D NoC Router Health & FIFO status

// Circular DMA Buffer Descriptor (64-byte aligned, zero-copy AXI4)
typedef struct __attribute__((aligned(64))) {
    uint64_t host_phys_addr;   // Source/Destination physical RAM address in Host DDR5
    uint64_t torus_dest_addr;  // Target memory address in 4D-Torus SRAM/HBM bank
    uint32_t length_bytes;     // Burst size (up to 4 MB per descriptor)
    uint32_t flags;            // Bits: [0]=EOP, [1]=SOP, [2]=IRQ_ON_DONE, [3]=DIRECTION (0=H2D, 1=D2H)
    uint64_t completion_token; // Unique sequence identifier returned upon commit
    uint8_t  _reserved[32];    // Padding to 64 bytes cacheline
} cron_dma_desc_t;

// Opaque Device Handle
typedef struct cron_pcie_device cron_pcie_device_t;

// API Functions
cron_pcie_device_t* cron_pcie_open(uint32_t device_index);
int  cron_pcie_dma_write_sync(cron_pcie_device_t* dev, uint64_t torus_addr, const void* src, size_t len);
int  cron_pcie_dma_read_sync(cron_pcie_device_t* dev, void* dest, uint64_t torus_addr, size_t len);
int  cron_pcie_submit_burst(cron_pcie_device_t* dev, const cron_dma_desc_t* desc);
int  cron_pcie_poll_completion(cron_pcie_device_t* dev, uint32_t timeout_ms);
void cron_pcie_close(cron_pcie_device_t* dev);

#ifdef __cplusplus
}
#endif

#endif // CRON_PCIE_DMA_H
