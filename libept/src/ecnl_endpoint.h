#ifndef ECNL_ENDPOINT_H
#define ECNL_ENDPOINT_H

#include <unistd.h>
#include <stdbool.h>
#include <stdint.h>

// duplicates defn from ecnl_proto.h
typedef struct {
    uint32_t len;
    uint8_t *frame;
} ept_buf_desc_t;

typedef struct {
    uint32_t ept_module_id;
    void *ept_sock; // struct nl_sock *
    void *ept_esock; // struct nl_sock *
    char *ept_name;
    uint32_t ept_port_id;
    int ept_up_down;
} ecnl_endpoint_t;

extern int ecnl_init(bool debug);
extern ecnl_endpoint_t *ept_create(uint32_t port_id);
extern void ept_destroy(ecnl_endpoint_t *ept);

extern void ept_do_read_async(ecnl_endpoint_t *ept, ept_buf_desc_t *actual_buf);
extern void ept_do_read(ecnl_endpoint_t *ept, ept_buf_desc_t *actual_buf, int nsecs);
extern void ept_do_xmit(ecnl_endpoint_t *ept, ept_buf_desc_t *buf);
extern void ept_update(ecnl_endpoint_t *ept);

// events:
extern void ept_get_event(ecnl_endpoint_t *ept);

// debug:
extern void ept_dumpbuf(ecnl_endpoint_t *ept, char *tag, ept_buf_desc_t *buf);

#endif
