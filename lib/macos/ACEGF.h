#ifndef ACEGF_H
#define ACEGF_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define MlDsa44_PK_BYTES 1312

#define MlDsa44_SK_BYTES 2528

#define MlDsa44_SEED_BYTES 32

#define ACEGF_METHOD_GENERATE 1

#define ACEGF_METHOD_WEITOA 2

#define ACEGF_METHOD_REKEY 3

#define ACEGF_METHOD_VIEW 4

typedef struct ACEGF_Call {
    int32_t method;
    const char *input_1;
    const char *input_2;
    const char *input_3;
} ACEGF_Call;

typedef struct ACEGF_Result {
    int32_t code;
    const char *data;
    uint64_t data_len;
    uint8_t reserved[8];
} ACEGF_Result;

extern int32_t acegf_ml_dsa_44_keypair_from_seed(uint8_t *pk, uint8_t *sk, const uint8_t *seed);

char *acegf_version(void);

int32_t acegf_call(const struct ACEGF_Call *call, struct ACEGF_Result *out);

void acegf_free_result(struct ACEGF_Result *result);

#endif  /* ACEGF_H */
