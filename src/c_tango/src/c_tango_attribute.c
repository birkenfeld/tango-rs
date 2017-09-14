/******************************************************************************
 *
 * File:        c_tango_attribute.c
 * Project:     C/Rust client interface to Tango
 * Description:	Interface functions to access Tango attributes
 * Original:    November 2007
 * Author:      jensmeyer
 *
 * Adapted for tango-rs by Georg Brandl, 2015.
 *
 ******************************************************************************/

#include "c_tango.h"
#include <tango.h>

ErrorStack *tango_translate_exception(Tango::DevFailed& tango_exception);
static void convert_attribute_reading (Tango::DeviceAttribute& devattr, AttributeData *argout);
static void convert_attribute_writing (AttributeData *argin, Tango::DeviceAttribute& devattr);
static void convert_attr_query(Tango::AttributeInfo& tango_attr_info, AttributeInfo *attr_info);


ErrorStack *tango_read_attributes(void *proxy, VarStringArray *attr_names, AttributeDataList *argout) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    vector<Tango::DeviceAttribute> *devattr_list = 0;

    // copy the attribute names to a vector of string
    vector<string> names;
    for (uint32_t i = 0; i < attr_names->length; i++) {
        names.push_back(attr_names->sequence[i]);
    }

    try {
        devattr_list = dev->read_attributes(names);

        // allocate the AttributeDataList for the number of attributes returned
        INIT_SEQ(*argout, AttributeData, devattr_list->size());

        // loop over all returned attributes and convert the data
        for (uint32_t i = 0; i < devattr_list->size(); i++) {
            if ((*devattr_list)[i].has_failed())
                throw Tango::DevFailed((*devattr_list)[i].get_err_stack());
            convert_attribute_reading ((*devattr_list)[i], &argout->sequence[i]);
        }

        // the memory is copied, we can now free the returned data
        delete devattr_list;
    }
    catch (Tango::DevFailed &tango_exception) {
        if (devattr_list)
            delete devattr_list;
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_read_attribute(void *proxy, char *attr_name, AttributeData *argout) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    Tango::DeviceAttribute devattr;

    try {
        devattr = dev->read_attribute(attr_name);
        if (devattr.has_failed())
            throw Tango::DevFailed(devattr.get_err_stack());
        convert_attribute_reading(devattr, argout);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_write_attributes(void *proxy, AttributeDataList *argin) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    vector<Tango::DeviceAttribute> devattr_list(argin->length);

    try {
        for (uint32_t i = 0; i < argin->length; i++) {
            convert_attribute_writing(&argin->sequence[i], devattr_list[i]);
        }
        dev->write_attributes(devattr_list);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_write_attribute(void *proxy, AttributeData *argin) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    Tango::DeviceAttribute devattr;

    try {
        convert_attribute_writing(argin, devattr);
        dev->write_attribute(devattr);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


void tango_free_AttributeData (AttributeData *attribute_data) {
    free(attribute_data->name);  // from strdup

#define DELETE_SEQ(member) \
    if (attribute_data->attr_data.member.sequence)              \
        delete[] attribute_data->attr_data.member.sequence;     \
    break

    switch (attribute_data->data_type) {

    case DEV_BOOLEAN: DELETE_SEQ(bool_arr);
    case DEV_UCHAR:   DELETE_SEQ(char_arr);
    case DEV_SHORT:   DELETE_SEQ(short_arr);
    case DEV_USHORT:  DELETE_SEQ(ushort_arr);
    case DEV_LONG:    DELETE_SEQ(long_arr);
    case DEV_ULONG:   DELETE_SEQ(ulong_arr);
    case DEV_LONG64:  DELETE_SEQ(long64_arr);
    case DEV_ULONG64: DELETE_SEQ(ulong64_arr);
    case DEV_FLOAT:   DELETE_SEQ(float_arr);
    case DEV_DOUBLE:  DELETE_SEQ(double_arr);
    case DEV_STATE:   DELETE_SEQ(state_arr);

    case DEV_STRING:
        for (uint32_t i = 0; i < attribute_data->attr_data.string_arr.length; i++) {
            free(attribute_data->attr_data.string_arr.sequence[i]);  // from strdup
        }
        DELETE_SEQ(string_arr);

    case DEV_ENCODED:
        for (uint32_t i = 0; i < attribute_data->attr_data.encoded_arr.length; i++) {
            free(attribute_data->attr_data.encoded_arr.sequence[i].encoded_format);  // from strdup
            delete[] attribute_data->attr_data.encoded_arr.sequence[i].encoded_data;
        }
        DELETE_SEQ(encoded_arr);

    default:
        break;
    }
}


void tango_free_AttributeDataList(AttributeDataList *attribute_data_list) {
    for (uint32_t i = 0; i < attribute_data_list->length; i++) {
        tango_free_AttributeData(&attribute_data_list->sequence[i]);
    }
    delete[] attribute_data_list->sequence;
}


ErrorStack *tango_get_attribute_list(void *proxy, VarStringArray *attr_names) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    vector<string> *attr_list = 0;

    try {
        attr_list = dev->get_attribute_list();
        int nb_data = attr_list->size();

        // allocate sequence
        INIT_SEQ(*attr_names, char *, nb_data);

        // allocate strings and copy data
        for (int i = 0; i < nb_data; i++) {
            attr_names->sequence[i] = strdup((*attr_list)[i].c_str());
        }
        delete attr_list;
    }
    catch (Tango::DevFailed &tango_exception) {
        if (attr_list)
            delete attr_list;
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_get_attribute_config(void *proxy, VarStringArray *attr_names, AttributeInfoList *attr_info_list) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    vector<Tango::AttributeInfo> *tango_attr_info_list = 0;

    // copy the attribute names to a vector of string
    vector<string> names;
    for (uint32_t i = 0; i < attr_names->length; i++) {
        names.push_back(attr_names->sequence[i]);
    }

    try {
        tango_attr_info_list = dev->get_attribute_config(names);

        // allocate the AttributeInfoList for the number of attributes returned
        INIT_SEQ(*attr_info_list, AttributeInfo, tango_attr_info_list->size());

        // loop over all returned attributes and convert the data
        for (uint32_t i = 0; i < tango_attr_info_list->size(); i++) {
            convert_attr_query((*tango_attr_info_list)[i], &attr_info_list->sequence[i]);
        }
        delete tango_attr_info_list;
    }
    catch (Tango::DevFailed &tango_exception) {
        if (tango_attr_info_list)
            delete tango_attr_info_list;
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_attribute_list_query (void *proxy, AttributeInfoList *attr_info_list) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    vector<Tango::AttributeInfo> *tango_attr_info_list = 0;

    try {
        tango_attr_info_list = dev->attribute_list_query();

        // allocate the AttributeInfoList for the number of attributes returned
        INIT_SEQ(*attr_info_list, AttributeInfo, tango_attr_info_list->size());

        // loop over all returned attributes and convert the data
        for (uint32_t i = 0; i < tango_attr_info_list->size(); i++) {
            convert_attr_query ((*tango_attr_info_list)[i], &attr_info_list->sequence[i]);
        }
        delete tango_attr_info_list;
    }
    catch (Tango::DevFailed &tango_exception) {
        if (tango_attr_info_list)
            delete tango_attr_info_list;
        return tango_translate_exception(tango_exception);
    }
    return 0;
}

void tango_free_VarStringArray(VarStringArray *string_arr) {
    for (uint32_t i = 0; i < string_arr->length; i++) {
        free(string_arr->sequence[i]);  // from strdup
    }
    delete[] string_arr->sequence;
}


void tango_free_AttributeInfoList(AttributeInfoList *attribute_info_list) {
    for (uint32_t i = 0; i < attribute_info_list->length; i++) {
        free(attribute_info_list->sequence[i].name);  // from strdup
        free(attribute_info_list->sequence[i].description);
        free(attribute_info_list->sequence[i].label);
        free(attribute_info_list->sequence[i].unit);
        free(attribute_info_list->sequence[i].standard_unit);
        free(attribute_info_list->sequence[i].display_unit);
        free(attribute_info_list->sequence[i].format);
        free(attribute_info_list->sequence[i].min_value);
        free(attribute_info_list->sequence[i].max_value);
        free(attribute_info_list->sequence[i].min_alarm);
        free(attribute_info_list->sequence[i].max_alarm);
        free(attribute_info_list->sequence[i].writable_attr_name);
    }
    delete[] attribute_info_list->sequence;
}


void convert_attribute_reading(Tango::DeviceAttribute& devattr, AttributeData *argout) {
    // treat INVALID data quality
    if (devattr.get_quality() == Tango::ATTR_INVALID) {
        // just initialise the first datatype - this should be valid for
        // all data types in the union!
        argout->attr_data.bool_arr.length   = 0;
        argout->attr_data.bool_arr.sequence = NULL;
    } else {
        int nb_data;

        // get data type
        argout->data_type = (TangoDataType)devattr.get_type();
        argout->data_format = (AttrDataFormat)devattr.data_format;
        argout->nb_read = devattr.get_nb_read();

#define EXTRACT_ARRAY(member, tgtype, type)                             \
        {                                                               \
            Tango::tgtype *seq;                                         \
            devattr >> seq;                                             \
            nb_data = seq->length();                                    \
            argout->attr_data.member.length = seq->length();            \
            argout->attr_data.member.sequence = seq->get_buffer(true);  \
            delete seq;                                                 \
            break;                                                      \
        }

        switch (argout->data_type) {

        case DEV_BOOLEAN: EXTRACT_ARRAY(bool_arr, DevVarBooleanArray, bool);
        case DEV_UCHAR:   EXTRACT_ARRAY(char_arr, DevVarCharArray, uint8_t);
        case DEV_SHORT:   EXTRACT_ARRAY(short_arr, DevVarShortArray, int16_t);
        case DEV_USHORT:  EXTRACT_ARRAY(ushort_arr, DevVarUShortArray, uint16_t);
        case DEV_LONG:    EXTRACT_ARRAY(long_arr, DevVarLongArray, int32_t);
        case DEV_ULONG:   EXTRACT_ARRAY(ulong_arr, DevVarULongArray, uint32_t);
        case DEV_LONG64:  EXTRACT_ARRAY(long64_arr, DevVarLong64Array, int64_t);
        case DEV_ULONG64: EXTRACT_ARRAY(ulong64_arr, DevVarULong64Array, uint64_t);
        case DEV_FLOAT:   EXTRACT_ARRAY(float_arr, DevVarFloatArray, float);
        case DEV_DOUBLE:  EXTRACT_ARRAY(double_arr, DevVarDoubleArray, double);

        case DEV_STRING: {
            vector<string> string_vect;
            devattr >> string_vect;
            nb_data = string_vect.size();
            INIT_SEQ(argout->attr_data.string_arr, char *, nb_data);
            for (int i = 0; i < nb_data; i++) {
                argout->attr_data.string_arr.sequence[i] = strdup(string_vect[i].c_str());
            }
            break;
        }

        case DEV_STATE: {
            vector<Tango::DevState> state_vect;

            // the State attribute is not returning a sequence - 
            // check whether the attribute name is State!
            if (devattr.name == "State") {
                state_vect.resize(1);
                devattr >> state_vect[0];
            } else {
                devattr >> state_vect;
            }
            nb_data = state_vect.size();
            INIT_SEQ(argout->attr_data.state_arr, TangoDevState, nb_data);
            for (int i = 0; i < nb_data; i++) {
                argout->attr_data.state_arr.sequence[i] = (TangoDevState)state_vect[i];
            }
            break;
        }

        case DEV_ENCODED: {
            Tango::DevVarEncodedArray *encoded_vect;
            uint32_t nb_data;

            devattr >> encoded_vect;
            nb_data = encoded_vect->length();
            INIT_SEQ(argout->attr_data.encoded_arr, TangoDevEncoded, nb_data);

            // allocate the encoded structues and copy data
            for (uint32_t i = 0; i < nb_data; i++) {
                argout->attr_data.encoded_arr.sequence[i].encoded_format =
                    strdup((*encoded_vect)[i].encoded_format);
                argout->attr_data.encoded_arr.sequence[i].encoded_length =
                    (*encoded_vect)[i].encoded_data.length();
                // get the pointer to the buffer and take over the memory ("true" param)
                argout->attr_data.encoded_arr.sequence[i].encoded_data =
                    (uint8_t *)(*encoded_vect)[i].encoded_data.get_buffer(true);
            }
            break;
        }

        default:
            Tango::Except::throw_exception(
                "Data type error",
                "The requested data type is not implemented for attribute reading!",
                "c_tango_attribute.c::convert_attribute_reading()");
            break;
        }
    }

    // get quality factor
    argout->quality = (AttrQuality)devattr.get_quality();

    // copy timestamp
    argout->time_stamp.tv_sec = devattr.time.tv_sec;
    argout->time_stamp.tv_usec = devattr.time.tv_usec;

    // allocate attribute name
    argout->name = strdup(devattr.name.c_str());

    // get data dimension
    argout->dim_x = devattr.dim_x;
    argout->dim_y = devattr.dim_y;
}


void convert_attribute_writing(AttributeData *argin, Tango::DeviceAttribute& devattr) {
    // allocate a vector and copy the data

#define INSERT_ARRAY(member, type)                              \
    {                                                           \
        vector<type> arr(argin->attr_data.member.length);       \
        memcpy(arr.data(), argin->attr_data.member.sequence,    \
               sizeof(type) * argin->attr_data.member.length);  \
        devattr.insert(arr, argin->dim_x, argin->dim_y);        \
        break;                                                  \
    }

    switch (argin->data_type) {

    case DEV_BOOLEAN: {
        vector<bool> arr(argin->attr_data.bool_arr.length);
        for (uint32_t i = 0; i < argin->attr_data.bool_arr.length; i++) {
            arr[i] = argin->attr_data.bool_arr.sequence[i];
        }
        devattr.insert(arr, argin->dim_x, argin->dim_y);
        break;
    }

    case DEV_UCHAR:   INSERT_ARRAY(char_arr, uint8_t);
    case DEV_SHORT:   INSERT_ARRAY(short_arr, int16_t);
    case DEV_USHORT:  INSERT_ARRAY(ushort_arr, uint16_t);
    case DEV_LONG:    INSERT_ARRAY(long_arr, int32_t);
    case DEV_ULONG:   INSERT_ARRAY(ulong_arr, uint32_t);
    case DEV_LONG64:  INSERT_ARRAY(long64_arr, int64_t);
    case DEV_ULONG64: INSERT_ARRAY(ulong64_arr, uint64_t);
    case DEV_FLOAT:   INSERT_ARRAY(float_arr, float);
    case DEV_DOUBLE:  INSERT_ARRAY(double_arr, double);

    case DEV_STRING: {
        vector<string> string_arr(argin->attr_data.string_arr.length);
        for (uint32_t i = 0; i < argin->attr_data.string_arr.length; i++) {
            string_arr[i] = argin->attr_data.string_arr.sequence[i];
        }
        devattr.insert(string_arr, argin->dim_x, argin->dim_y);
        break;
    }

    case DEV_STATE: {
        vector<Tango::DevState> state_arr(argin->attr_data.state_arr.length);
        for (uint32_t i = 0; i < argin->attr_data.state_arr.length; i++) {
            state_arr[i] = (Tango::DevState)argin->attr_data.state_arr.sequence[i];
        }
        devattr.insert(state_arr, argin->dim_x, argin->dim_y);
        break;
    }

    case DEV_ENCODED: {
        // today encoded type is only available as SCALAR data type
        devattr.insert(argin->attr_data.encoded_arr.sequence[0].encoded_format,
                       argin->attr_data.encoded_arr.sequence[0].encoded_data,
                       argin->attr_data.encoded_arr.sequence[0].encoded_length);
        break;
    }

    default:
        Tango::Except::throw_exception(
            "Data type error",
            "The requested data type is not implemented for attribute writing!",
            "c_tango_attribute.c::convert_attribute_writing()");
        break;
    }

    // set attribute name
    devattr.set_name(argin->name);
}


static void convert_attr_query(Tango::AttributeInfo& tango_attr_info, AttributeInfo *attr_info) {
    attr_info->name = strdup(tango_attr_info.name.c_str());
    attr_info->description = strdup(tango_attr_info.description.c_str());
    attr_info->label = strdup(tango_attr_info.label.c_str());
    attr_info->unit = strdup(tango_attr_info.unit.c_str());
    attr_info->standard_unit = strdup(tango_attr_info.standard_unit.c_str());
    attr_info->display_unit = strdup(tango_attr_info.display_unit.c_str());
    attr_info->format = strdup(tango_attr_info.format.c_str());
    attr_info->min_value = strdup(tango_attr_info.min_value.c_str());
    attr_info->max_value = strdup(tango_attr_info.max_value.c_str());
    attr_info->min_alarm = strdup(tango_attr_info.min_alarm.c_str());
    attr_info->max_alarm = strdup(tango_attr_info.max_alarm.c_str());
    attr_info->writable_attr_name = strdup(tango_attr_info.writable_attr_name.c_str());

    attr_info->writable = (AttrWriteType)tango_attr_info.writable;
    attr_info->data_format = (AttrDataFormat)tango_attr_info.data_format;
    attr_info->data_type = (TangoDataType)tango_attr_info.data_type;
    attr_info->max_dim_x = tango_attr_info.max_dim_x;
    attr_info->max_dim_y = tango_attr_info.max_dim_y;
    attr_info->disp_level = (DispLevel)tango_attr_info.disp_level;
}
