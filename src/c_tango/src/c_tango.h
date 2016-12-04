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
#ifndef __cplusplus
#include <stdbool.h>
#endif


typedef int TangoDevLong;
typedef unsigned int TangoDevULong;

#if __WORDSIZE == 64
typedef long TangoDevLong64;
typedef unsigned long TangoDevULong64;
#else
typedef long long TangoDevLong64;
typedef unsigned long long TangoDevULong64;
#endif


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
    unsigned int encoded_length;
    unsigned char *encoded_data;
} TangoDevEncoded;

typedef struct {
    unsigned int length;
    bool *sequence;
} VarBoolArray;

typedef struct {
    unsigned int length;
    unsigned char *sequence;
} VarCharArray;

typedef struct {
    unsigned int length;
    short *sequence;
} VarShortArray;

typedef struct {
    unsigned int length;
    unsigned short  *sequence;
} VarUShortArray;

typedef struct {
    unsigned int length;
    TangoDevLong *sequence;
} VarLongArray;

typedef struct {
    unsigned int length;
    TangoDevULong *sequence;
} VarULongArray;

typedef struct {
    unsigned int length;
    TangoDevLong64 *sequence;
} VarLong64Array;

typedef struct {
    unsigned int length;
    TangoDevULong64 *sequence;
} VarULong64Array;

typedef struct {
    unsigned int length;
    float *sequence;
} VarFloatArray;

typedef struct {
    unsigned int length;
    double *sequence;
} VarDoubleArray;

typedef struct {
    unsigned int length;
    char **sequence;
} VarStringArray;

typedef struct {
    unsigned int length;
    TangoDevState  *sequence;
} VarStateArray;

typedef struct {
    unsigned int length;
    TangoDevEncoded *sequence;
} VarEncodedArray;

typedef struct {
    unsigned int long_length;
    TangoDevLong *long_sequence;
    unsigned int string_length;
    char **string_sequence;
} VarLongStringArray;

typedef struct {
    unsigned int double_length;
    double *double_sequence;
    unsigned int string_length;
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
    short short_val;
    unsigned short ushort_val;
    int long_val;
    unsigned int ulong_val;
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
    unsigned char char_val;
    short short_val;
    unsigned short ushort_val;
    int long_val;
    unsigned int ulong_val;
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
    int dim_x;
    int dim_y;
    struct timeval time_stamp;
} AttributeData;

typedef struct {
    unsigned int length;
    AttributeData *sequence;
} AttributeDataList;

typedef struct {
    char *desc;
    char *reason;
    char *origin;
    ErrSeverity severity;
} DevFailed;

typedef struct {
    unsigned int length;
    DevFailed *sequence;
} ErrorStack;

typedef struct {
    char *cmd_name;
    int cmd_tag;
    int in_type;
    int out_type;
    char *in_type_desc;
    char *out_type_desc;
    DispLevel disp_level;
} CommandInfo;

typedef struct {
    unsigned int length;
    CommandInfo *sequence;
} CommandInfoList;

typedef struct {
    char *name;
    AttrWriteType writable;
    AttrDataFormat data_format;
    TangoDataType data_type;
    int max_dim_x;
    int max_dim_y;
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
    unsigned int length;
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
    unsigned int length;
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
