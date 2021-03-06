/******************************************************************************
 *
 * File:        c_tango.h
 * Project:     C client interface to Tango
 * Description: Definitions necessay to write Tango clients in C
 * Original:    November 2007
 * Author:      jensmeyer
 *
 * Adapted for tango-rs by Georg Brandl, 2015.
 *
 ******************************************************************************/

#ifndef C_TANGO_H
#define C_TANGO_H

#include <sys/time.h>
#include <stdint.h>
#ifndef __cplusplus
#include <stdbool.h>
#endif

#ifdef RUST_BINDGEN
/* Don't pull in tango.h for bindgen, these are the fixed sizes anyway. */
typedef int32_t TangoDevLong;
typedef uint32_t TangoDevULong;
typedef int64_t TangoDevLong64;
typedef uint64_t TangoDevULong64;
#else
#include <tango.h>
typedef Tango::DevLong TangoDevLong;
typedef Tango::DevULong TangoDevULong;
typedef Tango::DevLong64 TangoDevLong64;
typedef Tango::DevULong64 TangoDevULong64;
#endif


#define INIT_SEQ(seq, type, size)               \
    (seq).length = size;                        \
    (seq).sequence = new type[size]


typedef enum {
    DEV_VOID = 0,
    DEV_BOOLEAN,
    DEV_SHORT,
    DEV_LONG,
    DEV_FLOAT,
    DEV_DOUBLE,
    DEV_USHORT,
    DEV_ULONG,
    DEV_STRING,
    DEVVAR_CHARARRAY,
    DEVVAR_SHORTARRAY,
    DEVVAR_LONGARRAY,
    DEVVAR_FLOATARRAY,
    DEVVAR_DOUBLEARRAY,
    DEVVAR_USHORTARRAY,
    DEVVAR_ULONGARRAY,
    DEVVAR_STRINGARRAY,
    DEVVAR_LONGSTRINGARRAY,
    DEVVAR_DOUBLESTRINGARRAY,
    DEV_STATE,
    CONST_DEV_STRING,
    DEVVAR_BOOLEANARRAY,
    DEV_UCHAR,
    DEV_LONG64,
    DEV_ULONG64,
    DEVVAR_LONG64ARRAY,
    DEVVAR_ULONG64ARRAY,
    DEV_INT,
    DEV_ENCODED
} TangoDataType;

typedef enum {
    ON,
    OFF,
    CLOSE,
    OPEN,
    INSERT,
    EXTRACT,
    MOVING,
    STANDBY,
    FAULT,
    INIT,
    RUNNING,
    ALARM,
    DISABLE,
    UNKNOWN
} TangoDevState;

typedef enum {
    ATTR_VALID,
    ATTR_INVALID,
    ATTR_ALARM,
    ATTR_CHANGING,
    ATTR_WARNING
} AttrQuality;

typedef enum {
    READ,
    READ_WITH_WRITE,
    WRITE,
    READ_WRITE
} AttrWriteType;

typedef enum {
    SCALAR,
    SPECTRUM,
    IMAGE
} AttrDataFormat;

typedef enum {
    OPERATOR,
    EXPERT
} DispLevel;

typedef enum {
    WARN,
    ERR,
    PANIC
} ErrSeverity;

typedef enum {
    DEV,
    CACHE,
    CACHE_DEV
} DevSource;

typedef struct {
    char *encoded_format;
    uint32_t encoded_length;
    uint8_t *encoded_data;
} TangoDevEncoded;

typedef struct {
    uint32_t length;
    bool *sequence;
} VarBoolArray;

typedef struct {
    uint32_t length;
    uint8_t *sequence;
} VarCharArray;

typedef struct {
    uint32_t length;
    int16_t *sequence;
} VarShortArray;

typedef struct {
    uint32_t length;
    uint16_t  *sequence;
} VarUShortArray;

typedef struct {
    uint32_t length;
    TangoDevLong *sequence;
} VarLongArray;

typedef struct {
    uint32_t length;
    TangoDevULong *sequence;
} VarULongArray;

typedef struct {
    uint32_t length;
    TangoDevLong64 *sequence;
} VarLong64Array;

typedef struct {
    uint32_t length;
    TangoDevULong64 *sequence;
} VarULong64Array;

typedef struct {
    uint32_t length;
    float *sequence;
} VarFloatArray;

typedef struct {
    uint32_t length;
    double *sequence;
} VarDoubleArray;

typedef struct {
    uint32_t length;
    char **sequence;
} VarStringArray;

typedef struct {
    uint32_t length;
    TangoDevState  *sequence;
} VarStateArray;

typedef struct {
    uint32_t length;
    TangoDevEncoded *sequence;
} VarEncodedArray;

typedef struct {
    uint32_t long_length;
    TangoDevLong *long_sequence;
    uint32_t string_length;
    char **string_sequence;
} VarLongStringArray;

typedef struct {
    uint32_t double_length;
    double *double_sequence;
    uint32_t string_length;
    char **string_sequence;
} VarDoubleStringArray;

typedef union {
    VarBoolArray bool_arr;
    VarCharArray char_arr;
    VarShortArray short_arr;
    VarUShortArray ushort_arr;
    VarLongArray long_arr;
    VarULongArray ulong_arr;
    VarLong64Array long64_arr;
    VarULong64Array ulong64_arr;
    VarFloatArray float_arr;
    VarDoubleArray double_arr;
    VarStringArray string_arr;
    VarStateArray state_arr;
    VarEncodedArray encoded_arr;
} TangoAttributeData;

typedef union {
    bool bool_val;
    int16_t short_val;
    uint16_t ushort_val;
    int32_t long_val;
    uint32_t ulong_val;
    float float_val;
    double double_val;
    char *string_val;
    TangoDevState state_val;
    TangoDevLong64 long64_val;
    TangoDevULong64 ulong64_val;
    VarBoolArray bool_arr;
    VarCharArray char_arr;
    VarShortArray short_arr;
    VarUShortArray ushort_arr;
    VarLongArray long_arr;
    VarULongArray ulong_arr;
    VarLong64Array long64_arr;
    VarULong64Array ulong64_arr;
    VarFloatArray float_arr;
    VarDoubleArray double_arr;
    VarStringArray string_arr;
    VarStateArray state_arr;
    TangoDevEncoded encoded_val;
    VarLongStringArray long_string_arr;
    VarDoubleStringArray double_string_arr;
} TangoCommandData;

typedef union {
    bool bool_val;
    uint8_t char_val;
    int16_t short_val;
    uint16_t ushort_val;
    int32_t long_val;
    uint32_t ulong_val;
    float float_val;
    double double_val;
    char *string_val;
    TangoDevLong64 long64_val;
    TangoDevULong64 ulong64_val;

    VarShortArray short_arr;
    VarUShortArray ushort_arr;
    VarLongArray long_arr;
    VarULongArray ulong_arr;
    VarLong64Array long64_arr;
    VarULong64Array ulong64_arr;
    VarFloatArray float_arr;
    VarDoubleArray double_arr;
    VarStringArray string_arr;
} TangoPropertyData;

typedef struct {
    TangoDataType arg_type;
    TangoCommandData cmd_data;
} CommandData;

typedef struct {
    TangoDataType data_type;
    TangoAttributeData attr_data;
    AttrDataFormat data_format;
    AttrQuality quality;
    long nb_read;
    char *name;
    int32_t dim_x;
    int32_t dim_y;
    struct timeval time_stamp;
} AttributeData;

typedef struct {
    uint32_t length;
    AttributeData *sequence;
} AttributeDataList;

typedef struct {
    char *desc;
    char *reason;
    char *origin;
    ErrSeverity severity;
} DevFailed;

typedef struct {
    uint32_t length;
    DevFailed *sequence;
} ErrorStack;

typedef struct {
    char *cmd_name;
    int32_t cmd_tag;
    int32_t in_type;
    int32_t out_type;
    char *in_type_desc;
    char *out_type_desc;
    DispLevel disp_level;
} CommandInfo;

typedef struct {
    uint32_t length;
    CommandInfo *sequence;
} CommandInfoList;

typedef struct {
    char *name;
    AttrWriteType writable;
    AttrDataFormat data_format;
    TangoDataType data_type;
    int32_t max_dim_x;
    int32_t max_dim_y;
    char *description;
    char *label;
    char *unit;
    char *standard_unit;
    char *display_unit;
    char *format;
    char *min_value;
    char *max_value;
    char *min_alarm;
    char *max_alarm;
    char *writable_attr_name;
    DispLevel disp_level;
} AttributeInfo;

typedef struct {
    uint32_t length;
    AttributeInfo *sequence;
} AttributeInfoList;

typedef struct {
    char *property_name;
    TangoDataType data_type;
    TangoPropertyData prop_data;
    bool is_empty;
    bool wrong_data_type;
} DbDatum;

typedef struct {
    uint32_t length;
    DbDatum *sequence;
} DbData;


#ifdef __cplusplus
extern "C" {
#endif

ErrorStack *tango_create_device_proxy(char *dev_name, void **proxy);
ErrorStack *tango_delete_device_proxy(void *proxy);
ErrorStack *tango_set_timeout_millis(void *proxy, int millis);
ErrorStack *tango_get_timeout_millis(void *proxy, int *millis);
ErrorStack *tango_set_source(void *proxy, DevSource source);
ErrorStack *tango_get_source(void *proxy, DevSource *source);
ErrorStack *tango_lock(void *proxy);
ErrorStack *tango_unlock(void *proxy);
ErrorStack *tango_is_locked(void *proxy, bool *is_locked);
ErrorStack *tango_is_locked_by_me(void *proxy, bool *is_locked_by_me);
ErrorStack *tango_locking_status(void *proxy, char **lock_status);

ErrorStack *tango_command_query(void *proxy, char *cmd_name, CommandInfo *cmd_info);
ErrorStack *tango_command_list_query(void *proxy, CommandInfoList *cmd_info_list);
ErrorStack *tango_command_inout(void *proxy, char *cmd_name, CommandData *argin, CommandData *argout);
void tango_free_CommandData(CommandData *command_data);
void tango_free_CommandInfo(CommandInfo *command_info);
void tango_free_CommandInfoList(CommandInfoList *command_info_list);

ErrorStack *tango_get_attribute_list(void *proxy, VarStringArray *attr_names);
ErrorStack *tango_get_attribute_config(void *proxy, VarStringArray *attr_names, AttributeInfoList *attr_info_list);
ErrorStack *tango_attribute_list_query (void *proxy, AttributeInfoList *attr_info_list);
ErrorStack *tango_read_attribute(void *proxy, char *attr_name, AttributeData *argout);
ErrorStack *tango_write_attribute(void *proxy, AttributeData *argin);
ErrorStack *tango_read_attributes(void *proxy, VarStringArray *attr_names, AttributeDataList *argout);
ErrorStack *tango_write_attributes(void *proxy, AttributeDataList *argin);
void tango_free_AttributeData(AttributeData *attribute_data);
void tango_free_AttributeDataList(AttributeDataList *attribute_data_list);
void tango_free_VarStringArray(VarStringArray *string_arr);
void tango_free_AttributeInfoList(AttributeInfoList *attribute_info_list);

void tango_free_ErrorStack(ErrorStack *error_stack);

ErrorStack *tango_create_database_proxy(void **db_proxy);
ErrorStack *tango_delete_database_proxy(void *db_proxy);
ErrorStack *tango_get_device_exported(void *db_proxy, char *name_filter, DbDatum *dev_list);
ErrorStack *tango_get_device_exported_for_class(void *db_proxy, char *class_name, DbDatum *dev_list);
ErrorStack *tango_get_object_list(void *db_proxy, char *name_filter, DbDatum *obj_list);
ErrorStack *tango_get_object_property_list(void *db_proxy, char *obj_name, char *name_filter, DbDatum *prop_list);
ErrorStack *tango_get_property(void *db_proxy, char *obj_name, DbData *prop_list);
ErrorStack *tango_put_property(void *db_proxy, char *obj_name, DbData *prop_list);
ErrorStack *tango_delete_property(void *db_proxy, char *obj_name, DbData *prop_list);
ErrorStack *tango_get_device_property(void *dev_proxy, DbData *prop_list);
ErrorStack *tango_put_device_property(void *dev_proxy, DbData *prop_list);
ErrorStack *tango_delete_device_property(void *dev_proxy, DbData *prop_list);
void tango_free_DbDatum(DbDatum *db_datum);
void tango_free_DbData(DbData *db_data);

#ifdef __cplusplus
}      /* extern "C" */
#endif

#endif /* C_TANGO_H */
