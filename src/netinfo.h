#ifndef _NETINFO_H
#define _NETINFO_H 1

typedef struct {
    uint32_t sent;
    uint32_t recv;
} nettraffic_t;

nettraffic_t measure_network_traffic(uint32_t duration);

#endif // _NETINFO_H
