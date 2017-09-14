/******************************************************************************
 *
 * File:        c_tango_dbase.c
 * Project:     C client interface to Tango
 * Description: Interface functions to access Tango properties
 * Original:    November 2007
 * Author:      jensmeyer
 *
 * Adapted for tango-rs by Georg Brandl, 2015.
 *
 ******************************************************************************/

#include "c_tango.h"
#include <tango.h>

ErrorStack *tango_translate_exception(Tango::DevFailed& tango_exception);
static void convert_property_reading(Tango::DbDatum& tango_prop, DbDatum *prop);
static void convert_property_writing(DbDatum *prop, Tango::DbDatum& tango_prop);

ErrorStack *tango_create_database_proxy(void **db_proxy) {
    try {
        Tango::Database *dbase = new Tango::Database();
        *db_proxy = (void *)dbase;
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_delete_database_proxy(void *db_proxy) {
    Tango::Database *dbase = (Tango::Database *)db_proxy;

    try {
        delete dbase;
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_get_device_exported(void *db_proxy, char *name_filter, DbDatum *dev_list) {
    Tango::Database *dbase = (Tango::Database *)db_proxy;
    Tango::DbDatum tango_dev_list;

    try {
        string filter = name_filter;
        tango_dev_list = dbase->get_device_exported(filter);

        // the result is a string array, set the data type for the conversion
        dev_list->data_type = DEVVAR_STRINGARRAY;
        convert_property_reading(tango_dev_list, dev_list);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_get_device_exported_for_class(void *db_proxy, char *class_name, DbDatum *dev_list) {
    Tango::Database *dbase = (Tango::Database *)db_proxy;
    Tango::DbDatum tango_dev_list;

    try {
        string name = class_name;
        tango_dev_list = dbase->get_device_exported_for_class(name);

        // the result is a string array, set the data type for the conversion
        dev_list->data_type = DEVVAR_STRINGARRAY;
        convert_property_reading(tango_dev_list, dev_list);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_get_object_list(void *db_proxy, char *name_filter, DbDatum *obj_list) {
    Tango::Database *dbase = (Tango::Database *)db_proxy;
    Tango::DbDatum tango_obj_list;

    try {
        string filter = name_filter;
        tango_obj_list = dbase->get_object_list(filter);

        // the result is a string array, set the data type for the conversion
        obj_list->data_type = DEVVAR_STRINGARRAY;
        convert_property_reading(tango_obj_list, obj_list);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_get_object_property_list(void *db_proxy, char *obj_name, char *name_filter, DbDatum *prop_list) {
    Tango::Database *dbase = (Tango::Database *) db_proxy;
    Tango::DbDatum tango_prop_list;

    try {
        string name = obj_name;
        string filter = name_filter;
        tango_prop_list = dbase->get_object_property_list(name, filter);

        // the result is a string array, set the data type for the conversion
        prop_list->data_type = DEVVAR_STRINGARRAY;
        convert_property_reading(tango_prop_list, prop_list);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_get_property(void *db_proxy, char *obj_name, DbData *prop_list) {
    Tango::Database *dbase = (Tango::Database *)db_proxy;
    Tango::DbData tango_prop_list;

    try {
        string name = obj_name;

        // copy the property names into the Tango object
        for (uint32_t i = 0; i < prop_list->length; i++) {
            tango_prop_list.push_back(Tango::DbDatum(prop_list->sequence[i].property_name));
        }

        // read the properties
        dbase->get_property(name, tango_prop_list);

        for (uint32_t i = 0; i < prop_list->length; i++) {
            // copy the property data into the C structure
            convert_property_reading(tango_prop_list[i], &prop_list->sequence[i]);
        }
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_put_property(void *db_proxy, char *obj_name, DbData *prop_list) {
    Tango::Database *dbase = (Tango::Database *)db_proxy;
    Tango::DbData tango_prop_list;

    try {
        string name = obj_name;

        // copy the property names and data into the Tango object
        tango_prop_list.resize(prop_list->length);
        for (uint32_t i = 0; i < prop_list->length; i++) {
            convert_property_writing(&prop_list->sequence[i], tango_prop_list[i]);
        }

        // write the properties
        dbase->put_property(name, tango_prop_list);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_delete_property(void *db_proxy, char *obj_name, DbData *prop_list) {
    Tango::Database *dbase = (Tango::Database *)db_proxy;
    Tango::DbData tango_prop_list;

    try {
        string name = obj_name;

        // copy the property names into the Tango object
        for (uint32_t i = 0; i < prop_list->length; i++) {
            tango_prop_list.push_back(Tango::DbDatum(prop_list->sequence[i].property_name));
        }

        // read the properties
        dbase->delete_property(name, tango_prop_list);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_get_device_property(void *proxy, DbData *prop_list) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    Tango::DbData tango_prop_list;

    try {
        // copy the property names into the Tango object
        for (uint32_t i = 0; i < prop_list->length; i++) {
            tango_prop_list.push_back(Tango::DbDatum(prop_list->sequence[i].property_name));
        }

        // read the properties
        dev->get_property(tango_prop_list);

        for (uint32_t i = 0; i < prop_list->length; i++) {
            // copy the property data into the C structure
            convert_property_reading(tango_prop_list[i], &prop_list->sequence[i]);
        }
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_put_device_property(void *proxy, DbData *prop_list) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    Tango::DbData tango_prop_list;

    try {
        // copy the property names into the Tango object
        tango_prop_list.resize(prop_list->length);

        for (uint32_t i = 0; i < prop_list->length; i++) {
            convert_property_writing(&prop_list->sequence[i], tango_prop_list[i]);
        }

        // write the properties
        dev->put_property(tango_prop_list);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


ErrorStack *tango_delete_device_property(void *proxy, DbData *prop_list) {
    Tango::DeviceProxy *dev = (Tango::DeviceProxy *)proxy;
    Tango::DbData tango_prop_list;

    try {
        // copy the property names into the Tango object
        for (uint32_t i = 0; i < prop_list->length; i++) {
            tango_prop_list.push_back(Tango::DbDatum(prop_list->sequence[i].property_name));
        }

        // read the properties
        dev->delete_property(tango_prop_list);
    }
    catch (Tango::DevFailed &tango_exception) {
        return tango_translate_exception(tango_exception);
    }
    return 0;
}


void tango_free_DbDatum(DbDatum *db_datum) {
    free(db_datum->property_name);  // from strdup
    
    switch (db_datum->data_type) {
    case DEV_STRING:
        free(db_datum->prop_data.string_val);
        break;

    case DEVVAR_SHORTARRAY:
        delete[] db_datum->prop_data.short_arr.sequence;
        break;

    case DEVVAR_USHORTARRAY:
        delete[] db_datum->prop_data.ushort_arr.sequence;
        break;

    case DEVVAR_LONGARRAY:
        delete[] db_datum->prop_data.long_arr.sequence;
        break;

    case DEVVAR_ULONGARRAY:
        delete[] db_datum->prop_data.ulong_arr.sequence;
        break;

    case DEVVAR_LONG64ARRAY:
        delete[] db_datum->prop_data.long64_arr.sequence;
        break;

    case DEVVAR_ULONG64ARRAY:
        delete[] db_datum->prop_data.ulong64_arr.sequence;
        break;

    case DEVVAR_FLOATARRAY:
        delete[] db_datum->prop_data.float_arr.sequence;
        break;

    case DEVVAR_DOUBLEARRAY:
        delete[] db_datum->prop_data.double_arr.sequence;
        break;

    case DEVVAR_STRINGARRAY:
        for (uint32_t i = 0; i < db_datum->prop_data.string_arr.length; i++) {
            free(db_datum->prop_data.string_arr.sequence[i]);  // from strdup
        }

        delete[] db_datum->prop_data.string_arr.sequence;
        break;

    default:
        break;
    }
}


void tango_free_DbData(DbData *db_data) {
    for (uint32_t i = 0; i < db_data->length; i++) {
        tango_free_DbDatum(&db_data->sequence[i]);
    }
}


static void convert_property_reading(Tango::DbDatum &tango_prop, DbDatum *prop) {
    // allocate property name
    prop->property_name = strdup(tango_prop.name.c_str());

    // copy the property data into the C structure
    if (!tango_prop.is_empty()) {
        // set the flags
        prop->is_empty = false;
        prop->wrong_data_type = false;

        // convert the data
        switch (prop->data_type) {
        case DEV_BOOLEAN:
            if (!(tango_prop >> prop->prop_data.bool_val))
                prop->wrong_data_type = true;
            break;

        case DEV_UCHAR:
            if (!(tango_prop >> prop->prop_data.char_val))
                prop->wrong_data_type = true;
            break;

        case DEV_SHORT:
            if (!(tango_prop >> prop->prop_data.short_val))
                prop->wrong_data_type = true;
            break;

        case DEV_USHORT:
            if (!(tango_prop >> prop->prop_data.ushort_val))
                prop->wrong_data_type = true;
            break;

        case DEV_LONG: {
            Tango::DevLong long_val;
            if (!(tango_prop >> long_val))
                prop->wrong_data_type = true;
            else
                prop->prop_data.long_val = long_val;
            break;
        }

        case DEV_ULONG: {
            Tango::DevULong ulong_val;
            if (!(tango_prop >> ulong_val))
                prop->wrong_data_type = true;
            else
                prop->prop_data.ulong_val = ulong_val;
            break;
        }

        case DEV_LONG64: {
            Tango::DevLong64 long64_val;
            if (!(tango_prop >> long64_val))
                prop->wrong_data_type = true;
            else
                prop->prop_data.long64_val = long64_val;
            break;
        }

        case DEV_ULONG64: {
            Tango::DevULong64 ulong64_val;
            if (!(tango_prop >> ulong64_val))
                prop->wrong_data_type = true;
            else
                prop->prop_data.ulong64_val = ulong64_val;
            break;
        }

        case DEV_FLOAT:
            if (!(tango_prop >> prop->prop_data.float_val))
                prop->wrong_data_type = true;
            break;

        case DEV_DOUBLE:
            if (!(tango_prop >> prop->prop_data.double_val))
                prop->wrong_data_type = true;
            break;

        case DEV_STRING: {
            string string_val;
            if (tango_prop >> string_val) {
                prop->prop_data.string_val = strdup(string_val.c_str());
            } else {
                prop->wrong_data_type = true;
            }
            break;
        }

#define EXTRACT_ARRAY(member, type)                                     \
            {                                                           \
                vector<type> vect;                                      \
                uint32_t nb_data;                                       \
                if (tango_prop >> vect) {                               \
                    nb_data = vect.size();                              \
                    prop->prop_data.member.length = nb_data;            \
                    prop->prop_data.member.sequence = new type[nb_data]; \
                    memcpy(prop->prop_data.member.sequence,             \
                           vect.data(), sizeof(type) * nb_data);        \
                } else {                                                \
                    prop->wrong_data_type = true;                       \
                }                                                       \
                break;                                                  \
            }

        case DEVVAR_SHORTARRAY:   EXTRACT_ARRAY(short_arr, int16_t);
        case DEVVAR_USHORTARRAY:  EXTRACT_ARRAY(ushort_arr, uint16_t);
        case DEVVAR_LONGARRAY:    EXTRACT_ARRAY(long_arr, int32_t);
        case DEVVAR_ULONGARRAY:   EXTRACT_ARRAY(ulong_arr, uint32_t);
        case DEVVAR_LONG64ARRAY:  EXTRACT_ARRAY(long64_arr, int64_t);
        case DEVVAR_ULONG64ARRAY: EXTRACT_ARRAY(ulong64_arr, uint64_t);
        case DEVVAR_FLOATARRAY:   EXTRACT_ARRAY(float_arr, float);
        case DEVVAR_DOUBLEARRAY:  EXTRACT_ARRAY(double_arr, double);

        case DEVVAR_STRINGARRAY: {
            vector<string> string_vect;
            uint32_t nb_data;

            if (tango_prop >> string_vect) {
                nb_data = string_vect.size();

                prop->prop_data.string_arr.sequence = new char *[nb_data];
                prop->prop_data.string_arr.length = nb_data;

                for (uint32_t i = 0; i < nb_data; i++) {
                    prop->prop_data.string_arr.sequence[i] = strdup(string_vect[i].c_str());
                }
            } else {
                prop->wrong_data_type = true;
            }
            break;
        }

        default:
            Tango::Except::throw_exception(
                "Data type error",
                "The requested data type is not implemented for property reading!",
                "c_tango_dbase.c::convert_property_reading()");
            break;
        }
    } else {
        // no property value found, set the is_empty flag
        prop->is_empty = true;
        prop->wrong_data_type = false;
    }
}


static void convert_property_writing(DbDatum *prop, Tango::DbDatum& tango_prop) {
    tango_prop.name = prop->property_name;

    switch (prop->data_type) {
    case DEV_BOOLEAN:
        tango_prop << prop->prop_data.bool_val;
        break;

    case DEV_UCHAR:
        tango_prop << prop->prop_data.char_val;
        break;

    case DEV_SHORT:
        tango_prop << prop->prop_data.short_val;
        break;

    case DEV_USHORT:
        tango_prop << prop->prop_data.ushort_val;
        break;

    case DEV_LONG:
        tango_prop << (Tango::DevLong)prop->prop_data.long_val;
        break;

    case DEV_ULONG:
        tango_prop << (Tango::DevULong)prop->prop_data.ulong_val;
        break;

    case DEV_LONG64:
        tango_prop << (Tango::DevLong64)prop->prop_data.long64_val;
        break;

    case DEV_ULONG64:
        tango_prop << (Tango::DevULong64)prop->prop_data.ulong64_val;
        break;

    case DEV_FLOAT:
        tango_prop << prop->prop_data.float_val;
        break;

    case DEV_DOUBLE:
        tango_prop << prop->prop_data.double_val;
        break;

    case DEV_STRING:
    case CONST_DEV_STRING:
        tango_prop << prop->prop_data.string_val;
        break;

#define INSERT_ARRAY(member, type) \
        {                                                               \
            vector<type> arr(prop->prop_data.member.length);            \
            memcpy(arr.data(), prop->prop_data.member.sequence,         \
                   sizeof(type) * prop->prop_data.member.length);       \
            tango_prop << arr;                                          \
            break;                                                      \
        }

    case DEVVAR_SHORTARRAY:   INSERT_ARRAY(short_arr, int16_t);
    case DEVVAR_USHORTARRAY:  INSERT_ARRAY(ushort_arr, uint16_t);
    case DEVVAR_LONGARRAY:    INSERT_ARRAY(long_arr, int32_t);
    case DEVVAR_ULONGARRAY:   INSERT_ARRAY(ulong_arr, uint32_t);
    case DEVVAR_LONG64ARRAY:  INSERT_ARRAY(long64_arr, int64_t);
    case DEVVAR_ULONG64ARRAY: INSERT_ARRAY(ulong64_arr, uint64_t);
    case DEVVAR_FLOATARRAY:   INSERT_ARRAY(float_arr, float);
    case DEVVAR_DOUBLEARRAY:  INSERT_ARRAY(double_arr, double);

    case DEVVAR_STRINGARRAY: {
        vector<string> string_arr(prop->prop_data.string_arr.length);

        for (uint32_t i = 0; i < prop->prop_data.string_arr.length; i++) {
            string_arr[i] = prop->prop_data.string_arr.sequence[i];
        }

        tango_prop << string_arr;
        break;
    }

    default:
        Tango::Except::throw_exception(
            "Data type error",
            "The requested data type is not implemented for property writing!",
            "c_tango_dbase.c::convert_property_writing()");
        break;
    }
}
