static const char *RcsId = "$Id: c_tango_dbase.c 12014 2007-12-18 17:26:20Z jensmeyer $\n$Name$";
/******************************************************************************
 * 
 * File       :	c_tango_dbase.c
 *
 * Project    :	C client interface to Tango
 * 
 * Description:	Interface functions to access Tango properties
 *
 * Original   :	November 2007	
 *	
 * $Author: jensmeyer $
 *
 * $Log$
 *
 ******************************************************************************/ 

#include <c_tango.h>
#include <tango.h>

void convert_property_reading (Tango::DbDatum& tango_prop, DbDatum *prop);
void convert_property_writing (DbDatum *prop, Tango::DbDatum& tango_prop);
void translate_exception (Tango::DevFailed& tango_exception, ErrorStack *error);


/********************************/
/* External interface functions */
/********************************/

/*
 * Create and delete the access to the Tango database
 */

bool tango_create_database_proxy (void **db_proxy, ErrorStack *error)
{
	try
		{
		Tango::Database *dbase = new Tango::Database();
		*db_proxy = (void *) dbase;
		}
	
	catch (Tango::DevFailed &e)
		{
		translate_exception (e, error);
		return false;
		}
			
	return true;
}


bool tango_delete_database_proxy (void **db_proxy, ErrorStack *error)
{
	Tango::Database *dbase;
	
	try
		{
		dbase = (Tango::Database *) *db_proxy;
		delete dbase;
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
	
	*db_proxy = NULL;
	return true;
}

/*
 * Device query functions with wilcards
 */

bool tango_get_device_exported (void *db_proxy, char *name_filter, DbDatum *dev_list, ErrorStack *error)
{
	Tango::DbDatum tango_dev_list;
	Tango::Database *dbase;
	
	try
		{
		dbase = (Tango::Database *) db_proxy;
		
		string filter = name_filter;
		tango_dev_list = dbase->get_device_exported (filter);
		
		/* The result is a string array, set the data type for the conversion */
		dev_list->data_type = DEVVAR_STRINGARRAY;
		convert_property_reading (tango_dev_list, dev_list);
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
		
	return true;
}


bool tango_get_device_exported_for_class 
       (void *db_proxy, char *class_name, DbDatum *dev_list, ErrorStack *error)
{
	Tango::DbDatum tango_dev_list;
	Tango::Database *dbase;
	
	try
		{
		dbase = (Tango::Database *) db_proxy;
		
		string name = class_name;
		tango_dev_list = dbase->get_device_exported_for_class (name);
		
		/* The result is a string array, set the data type for the conversion */
		dev_list->data_type = DEVVAR_STRINGARRAY;
		convert_property_reading (tango_dev_list, dev_list);
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
		
	return true;
}

/* 
 * Funtions to handle free properties in the Tango database 
 */

bool tango_get_object_list (void *db_proxy, char *name_filter, 
                            DbDatum *obj_list, ErrorStack *error)
{
	Tango::DbDatum tango_obj_list;
	Tango::Database *dbase;
	
	try
		{
		dbase = (Tango::Database *) db_proxy;
		
		string filter = name_filter;
		tango_obj_list = dbase->get_object_list (filter);
		
		/* The result is a string array, set the data type for the conversion */
		obj_list->data_type = DEVVAR_STRINGARRAY;
		convert_property_reading (tango_obj_list, obj_list);
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
		
	return true;
}


bool tango_get_object_property_list (void *db_proxy, char *obj_name, char *name_filter, 
                                     DbDatum *prop_list, ErrorStack *error)
{	
	Tango::DbDatum tango_prop_list;
	Tango::Database *dbase;
	
	try
		{
		dbase = (Tango::Database *) db_proxy;
		
		string name   = obj_name;
		string filter = name_filter;
		tango_prop_list = dbase->get_object_property_list (name, filter);
		
		/* The result is a string array, set the data type for the conversion */
		prop_list->data_type = DEVVAR_STRINGARRAY;
		convert_property_reading (tango_prop_list, prop_list);
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
		
	return true;
}


bool tango_get_property  	(void *db_proxy, char *obj_name, DbData *prop_list, ErrorStack *error)
{
	Tango::DbData tango_prop_list;
	Tango::Database *dbase;
	
	try
		{
		dbase = (Tango::Database *) db_proxy;
		
		/* Copy the property names into the Tango object */
		
		for (int i=0; i<prop_list->length; i++)
			{
			tango_prop_list.push_back 
					(Tango::DbDatum (prop_list->sequence[i].property_name));
			}
		
		string name = obj_name;
		
		/* read the properties */
		dbase->get_property(name, tango_prop_list);
		
		for (int i=0; i<prop_list->length; i++)
			{		
			/* copy the property data into the C structure */
			convert_property_reading (tango_prop_list[i], &(prop_list->sequence[i]));
			}
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
		
	return true;
}


bool tango_put_property (void *db_proxy, char *obj_name, DbData *prop_list, ErrorStack *error)
{
	Tango::DbData tango_prop_list;
	Tango::Database *dbase;
	
	try
		{
		dbase = (Tango::Database *) db_proxy;
		
		/* Copy the property names and data into the Tango object */
		tango_prop_list.resize(prop_list->length);
		
		for (int i=0; i<prop_list->length; i++)
			{
			convert_property_writing (&(prop_list->sequence[i]), tango_prop_list[i]);
			}
		
		string name = obj_name;
		
		/* write the properties */
		dbase->put_property(name, tango_prop_list);
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
		
	return true;
}


bool tango_delete_property (void *db_proxy, char *obj_name, DbData *prop_list, ErrorStack *error)
{
	Tango::DbData tango_prop_list;
	Tango::Database *dbase;
	
	try
		{
		dbase = (Tango::Database *) db_proxy;
		
		/* Copy the property names into the Tango object */
		
		for (int i=0; i<prop_list->length; i++)
			{
			tango_prop_list.push_back 
					(Tango::DbDatum (prop_list->sequence[i].property_name));
			}
		
		string name = obj_name;
		
		/* read the properties */
		dbase->delete_property (name, tango_prop_list);
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
		
	return true;
}

/* 
 * Device property functions work with the device proxy instead the database proxy! 
 * This allows easier property access
 */

bool tango_get_device_property (void *proxy, DbData *prop_list, ErrorStack *error)
{
	Tango::DbData tango_prop_list;
	Tango::DeviceProxy *dev;
	
	try
		{
		dev = (Tango::DeviceProxy *) proxy;
		
		/* Copy the property names into the Tango object */
		
		for (int i=0; i<prop_list->length; i++)
			{
			tango_prop_list.push_back 
					(Tango::DbDatum (prop_list->sequence[i].property_name));
			}
		
		/* read the properties */
		dev->get_property(tango_prop_list);
		
		for (int i=0; i<prop_list->length; i++)
			{		
			/* copy the property data into the C structure */
			convert_property_reading (tango_prop_list[i], &(prop_list->sequence[i]));
			}
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
		
	return true;
}


bool tango_put_device_property  	 (void *proxy, DbData *prop_list, ErrorStack *error)
{
	Tango::DbData tango_prop_list;
	Tango::DeviceProxy *dev;
	
	try
		{
		dev = (Tango::DeviceProxy *) proxy;
		
		/* Copy the property names into the Tango object */
		tango_prop_list.resize(prop_list->length);
		
		for (int i=0; i<prop_list->length; i++)
			{
			convert_property_writing (&(prop_list->sequence[i]), tango_prop_list[i]);
			}		
		
		/* write the properties */
		dev->put_property(tango_prop_list);
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
		
	return true;
}


bool tango_delete_device_property  (void *proxy, DbData *prop_list, ErrorStack *error)
{
	Tango::DbData tango_prop_list;
	Tango::DeviceProxy *dev;
	
	try
		{
		dev = (Tango::DeviceProxy *) proxy;
		
		/* Copy the property names into the Tango object */
		
		for (int i=0; i<prop_list->length; i++)
			{
			tango_prop_list.push_back 
					(Tango::DbDatum (prop_list->sequence[i].property_name));
			}
		
		/* read the properties */
		dev->delete_property (tango_prop_list);
		}
		
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);
		return false;
		}
		
	return true;
}

/* 
 * Functions to free returned allocated database information 
 */

void tango_free_DbDatum (DbDatum *db_datum)
{
	free (db_datum->property_name);
	db_datum->property_name = NULL;
	
	switch (db_datum->data_type)
		{
		case DEV_STRING:
			free (db_datum->prop_data.string_val);
			break;
											
		case DEVVAR_SHORTARRAY:
			free (db_datum->prop_data.short_arr.sequence);
			
			db_datum->prop_data.short_arr.sequence = NULL;
			db_datum->prop_data.short_arr.length = 0;
			break;
	
		case DEVVAR_USHORTARRAY:
			free (db_datum->prop_data.ushort_arr.sequence);
			
			db_datum->prop_data.ushort_arr.sequence = NULL;
			db_datum->prop_data.ushort_arr.length = 0;
			break;	
			
		case DEVVAR_LONGARRAY:
			free (db_datum->prop_data.long_arr.sequence);
			
			db_datum->prop_data.long_arr.sequence = NULL;
			db_datum->prop_data.long_arr.length = 0;
			break;			
	
		case DEVVAR_ULONGARRAY:
			free (db_datum->prop_data.ulong_arr.sequence);
			
			db_datum->prop_data.ulong_arr.sequence = NULL;
			db_datum->prop_data.ulong_arr.length = 0;
			break;	

		case DEVVAR_LONG64ARRAY:
			free (db_datum->prop_data.long64_arr.sequence);
			
			db_datum->prop_data.long64_arr.sequence = NULL;
			db_datum->prop_data.long64_arr.length = 0;
			break;			
	
		case DEVVAR_ULONG64ARRAY:
			free (db_datum->prop_data.ulong64_arr.sequence);
			
			db_datum->prop_data.ulong64_arr.sequence = NULL;
			db_datum->prop_data.ulong64_arr.length = 0;
			break;
	
		case DEVVAR_FLOATARRAY:
			free (db_datum->prop_data.float_arr.sequence);
			
			db_datum->prop_data.float_arr.sequence = NULL;
			db_datum->prop_data.float_arr.length = 0;
			break;	
					
		case DEVVAR_DOUBLEARRAY:
			free (db_datum->prop_data.double_arr.sequence);
			
			db_datum->prop_data.double_arr.sequence = NULL;
			db_datum->prop_data.double_arr.length = 0;
			break;
			
		case DEVVAR_STRINGARRAY:
			for (int i=0; i<db_datum->prop_data.string_arr.length; i++ )
				{
				free (db_datum->prop_data.string_arr.sequence[i]);
				}
				
			free (db_datum->prop_data.string_arr.sequence);
			db_datum->prop_data.string_arr.sequence = NULL;
			db_datum->prop_data.string_arr.length = 0;
			break;
			}
}


void tango_free_DbData (DbData  *db_data)
{
	for (int i=0; i<db_data->length; i++)
		{
		tango_free_DbDatum ( &(db_data->sequence[i]) );
		}	
}


/*************************************/
/* internal library helper functions */
/*************************************/


void convert_property_reading (Tango::DbDatum& tango_prop, DbDatum *prop)
{
	/* allocate property name */
	
	prop->property_name = new char[tango_prop.name.length() + 1];
	sprintf (prop->property_name, "%s", tango_prop.name.c_str());
		
	/* copy the property data into the C structure */
		
	if ( tango_prop.is_empty() == false )
		{
		/* set the flags */
		prop->is_empty        = false;
		prop->wrong_data_type = false;

		/* Convert the data */
		switch ( prop->data_type )
			{
			case DEV_BOOLEAN:
				if ( !(tango_prop >> prop->prop_data.bool_val) )
					{
					prop->wrong_data_type = true;
					}
				break;				
			
			case DEV_UCHAR:
				if ( !(tango_prop >> prop->prop_data.char_val) )
					{
					prop->wrong_data_type = true;
					}
				break;
									
			case DEV_SHORT:
				if ( !(tango_prop >> prop->prop_data.short_val) )
					{
					prop->wrong_data_type = true;
					}
				break;
				
			case DEV_USHORT:
				if ( !(tango_prop >> prop->prop_data.ushort_val) )
					{
					prop->wrong_data_type = true;
					}
				break;								
		
			case DEV_LONG:
				{
				Tango::DevLong long_val;
				if ( !(tango_prop >> long_val) )
					{
					prop->wrong_data_type = true;
					}
				else
					prop->prop_data.long_val = long_val;
				break;
				}				

			case DEV_ULONG:
				{
				Tango::DevULong ulong_val;
				if ( !(tango_prop >> ulong_val) )
					{
					prop->wrong_data_type = true;
					}
				else
					prop->prop_data.ulong_val = ulong_val;
				break;
				}				
			
			case DEV_LONG64:
				{
				Tango::DevLong64 long64_val;
				if ( !(tango_prop >> long64_val) )
					{
					prop->wrong_data_type = true;
					}
				else
					prop->prop_data.long64_val = long64_val;
				break;
				}				

			case DEV_ULONG64:
				{
				Tango::DevULong64 ulong64_val;
				if ( !(tango_prop >> ulong64_val) )
					{
					prop->wrong_data_type = true;
					}
				else
					prop->prop_data.ulong64_val = ulong64_val;
				break;
				}		
			
			case DEV_FLOAT:
				if ( !(tango_prop >> prop->prop_data.float_val) )
					{
					prop->wrong_data_type = true;
					}
				break;			
			
			case DEV_DOUBLE:						
				if ( !(tango_prop >> prop->prop_data.double_val) )
					{
					prop->wrong_data_type = true;
					}
				break;
			
			case DEV_STRING:
				{
				string string_val;
				
				if ( (tango_prop >> string_val) )
					{
					prop->prop_data.string_val = new char[string_val.length() + 1];
					sprintf (prop->prop_data.string_val, "%s", string_val.c_str());
					}
				else
					{
					prop->wrong_data_type = true;
					}
				break;
				}
	
			case DEVVAR_SHORTARRAY:
				{
				vector<short>	short_vect;
				int nb_data;

				if ( (tango_prop >> short_vect) )
					{
					nb_data =  short_vect.size();

					prop->prop_data.short_arr.length   = nb_data;	
					prop->prop_data.short_arr.sequence = new short[nb_data];

					for (int i=0 ; i<nb_data ; i++)
						{
						prop->prop_data.short_arr.sequence[i] = short_vect[i];
						}
					}
				else
					{
					prop->wrong_data_type = true;
					}				
				break;
				}

			case DEVVAR_USHORTARRAY:
				{
				vector<unsigned short>	ushort_vect;
				int nb_data;

				if ( (tango_prop >> ushort_vect) )
					{
					nb_data =  ushort_vect.size();

					prop->prop_data.ushort_arr.length   = nb_data;	
					prop->prop_data.ushort_arr.sequence = new unsigned short[nb_data];

					for (int i=0 ; i<nb_data ; i++)
						{
						prop->prop_data.ushort_arr.sequence[i] = ushort_vect[i];
						}
					}
				else
					{
					prop->wrong_data_type = true;
					}						
				break;
				}				

			case DEVVAR_LONGARRAY:
				{
				vector<Tango::DevLong>	long_vect;
				int nb_data;

				if ( (tango_prop >> long_vect) )
					{
					nb_data =  long_vect.size();

					prop->prop_data.long_arr.length   = nb_data;	
					prop->prop_data.long_arr.sequence = new TangoDevLong[nb_data];

					for (int i=0 ; i<nb_data ; i++)
						{
						prop->prop_data.long_arr.sequence[i] = long_vect[i];
						}
					}
				else
					{
					prop->wrong_data_type = true;
					}					
				break;
				}

			case DEVVAR_ULONGARRAY:
				{
				vector<Tango::DevULong>	ulong_vect;
				int nb_data;

				if ( (tango_prop >> ulong_vect) )
					{
					nb_data =  ulong_vect.size();

					prop->prop_data.ulong_arr.length   = nb_data;	
					prop->prop_data.ulong_arr.sequence = new TangoDevULong[nb_data];

					for (int i=0 ; i<nb_data ; i++)
						{
						prop->prop_data.ulong_arr.sequence[i] = ulong_vect[i];
						}
					}
				else
					{
					prop->wrong_data_type = true;
					}				
				break;
				}
				
			case DEVVAR_LONG64ARRAY:
				{
				vector<Tango::DevLong64>	long64_vect;
				int nb_data;

				if ( (tango_prop >> long64_vect) )
					{
					nb_data =  long64_vect.size();

					prop->prop_data.long64_arr.length   = nb_data;	
					prop->prop_data.long64_arr.sequence = new TangoDevLong64[nb_data];

					for (int i=0 ; i<nb_data ; i++)
						{
						prop->prop_data.long64_arr.sequence[i] = long64_vect[i];
						}
					}
				else
					{
					prop->wrong_data_type = true;
					}						
				break;
				}

			case DEVVAR_ULONG64ARRAY:
				{
				vector<Tango::DevULong64>	ulong64_vect;
				int nb_data;

				if ( (tango_prop >> ulong64_vect) )
					{
					nb_data =  ulong64_vect.size();

					prop->prop_data.ulong64_arr.length   = nb_data;	
					prop->prop_data.ulong64_arr.sequence = new TangoDevULong64[nb_data];

					for (int i=0 ; i<nb_data ; i++)
						{
						prop->prop_data.ulong64_arr.sequence[i] = ulong64_vect[i];
						}
					}
				else
					{
					prop->wrong_data_type = true;
					}						
				break;
				}				
				
			case DEVVAR_FLOATARRAY:
				{
				vector<float>	float_vect;
				int nb_data;

				if ( (tango_prop >> float_vect) )
					{
					nb_data =  float_vect.size();

					prop->prop_data.float_arr.length   = nb_data;	
					prop->prop_data.float_arr.sequence = new float[nb_data];

					for (int i=0 ; i<nb_data ; i++)
						{
						prop->prop_data.float_arr.sequence[i] = float_vect[i];
						}
					}
				else
					{
					prop->wrong_data_type = true;
					}				
				break;
				}			
				
			case DEVVAR_DOUBLEARRAY:
				{
				vector<double>	double_vect;
				int nb_data;

				if ( (tango_prop >> double_vect) )
					{
					nb_data =  double_vect.size();

					prop->prop_data.double_arr.length   = nb_data;	
					prop->prop_data.double_arr.sequence = new double[nb_data];

					for (int i=0 ; i<nb_data ; i++)
						{
						prop->prop_data.double_arr.sequence[i] = double_vect[i];
						}
					}
				else
					{
					prop->wrong_data_type = true;
					}				
				break;
				}	
				
			case DEVVAR_STRINGARRAY:
				{
				vector<string>	string_vect;
				int nb_data;

				if ( (tango_prop >> string_vect) )
					{
					nb_data = string_vect.size();

					/* allocate sequence */
					prop->prop_data.string_arr.sequence = (char **) calloc(nb_data, sizeof(char *));;
					prop->prop_data.string_arr.length   = nb_data;

					/* allocate strings and copy data */
					for (int i=0 ; i<nb_data ; i++)
						{
						prop->prop_data.string_arr.sequence[i] = new char[string_vect[i].length() + 1];
						sprintf (prop->prop_data.string_arr.sequence[i], "%s", 
				         	   string_vect[i].c_str());
						}
					}
				else
					{
					prop->wrong_data_type = true;
					}				  
				break;										
				}

			default:
				Tango::Except::throw_exception 
						((const char *)"Data type error",
	                (const char *)"The requested data type is not implemented for property reading!",
	                (const char *)"c_tango_dbase.c::convert_property_reading()");
				break;
			}
		}
	else
		{
		/* No property value found. Set the is_empty flag! */
		prop->is_empty        = true;
		prop->wrong_data_type = false;
		}
}

void convert_property_writing (DbDatum *prop, Tango::DbDatum& tango_prop)
{
	tango_prop.name = prop->property_name;

	switch ( prop->data_type )
		{	
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
			 tango_prop << (Tango::DevLong) prop->prop_data.long_val;
			 break;				

		 case DEV_ULONG:
			 tango_prop << (Tango::DevULong) prop->prop_data.ulong_val;
			 break;				

		 case DEV_LONG64:
			 tango_prop << (Tango::DevLong64) prop->prop_data.long64_val;
			 break;				

		 case DEV_ULONG64:
			 tango_prop << (Tango::DevULong64) prop->prop_data.ulong64_val;
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

		 case DEVVAR_SHORTARRAY:
			 {
			 vector<short> short_arr(prop->prop_data.short_arr.length);

			 for (int i=0; i<prop->prop_data.short_arr.length; i++)
				 {
				 short_arr[i] = (short) prop->prop_data.short_arr.sequence[i];
				 }

			 tango_prop << short_arr;				
			 break;
			 }

		 case DEVVAR_USHORTARRAY:
			 {
			 vector<unsigned short> ushort_arr(prop->prop_data.ushort_arr.length);

			 for (int i=0; i<prop->prop_data.ushort_arr.length; i++)
				 {
				 ushort_arr[i] = (unsigned short) prop->prop_data.ushort_arr.sequence[i];
				 }

			 tango_prop << ushort_arr;				
			 break;
			 }

		 case DEVVAR_LONGARRAY:
			 {
			 vector<Tango::DevLong> long_arr(prop->prop_data.long_arr.length);

			 for (int i=0; i<prop->prop_data.long_arr.length; i++)
				 {
				 long_arr[i] = (int) prop->prop_data.long_arr.sequence[i];
				 }

			 tango_prop << long_arr;				
			 break;
			 }

		 case DEVVAR_ULONGARRAY:
			 {
			 vector<Tango::DevULong> ulong_arr(prop->prop_data.ulong_arr.length);

			 for (int i=0; i<prop->prop_data.ulong_arr.length; i++)
				 {
				 ulong_arr[i] = (unsigned int) prop->prop_data.ulong_arr.sequence[i];
				 }

			 tango_prop << ulong_arr;				
			 break;
			 }

		 case DEVVAR_LONG64ARRAY:				
			 {
			 vector<Tango::DevLong64> long64_arr(prop->prop_data.long64_arr.length);

			 for (int i=0; i<prop->prop_data.long64_arr.length; i++)
				 {
				 long64_arr[i] = (Tango::DevLong64) prop->prop_data.long64_arr.sequence[i];
				 }

			 tango_prop << long64_arr;				
			 break;
			 }

		 case DEVVAR_ULONG64ARRAY:			
			 {
			 vector<Tango::DevULong64> ulong64_arr(prop->prop_data.ulong64_arr.length);

			 for (int i=0; i<prop->prop_data.ulong64_arr.length; i++)
				 {
				 ulong64_arr[i] = (Tango::DevULong64) prop->prop_data.ulong64_arr.sequence[i];
				 }

			 tango_prop << ulong64_arr;				
			 break;																						
			 }

		 case DEVVAR_FLOATARRAY:
			 {
			 vector<float> float_arr(prop->prop_data.float_arr.length);

			 for (int i=0; i<prop->prop_data.float_arr.length; i++)
				 {
				 float_arr[i] = (float) prop->prop_data.float_arr.sequence[i];
				 }

			 tango_prop << float_arr;				
			 break;				
			 }

		 case DEVVAR_DOUBLEARRAY:
			 {
			 vector<double> double_arr(prop->prop_data.double_arr.length);

			 for (int i=0; i<prop->prop_data.double_arr.length; i++)
				 {
				 double_arr[i] = (double) prop->prop_data.double_arr.sequence[i];
				 }

			 tango_prop << double_arr;				
			 break;
			 }

		 case DEVVAR_STRINGARRAY:
			 {
			 vector<string> string_arr(prop->prop_data.string_arr.length);

			 for (int i=0; i<prop->prop_data.string_arr.length; i++)
				 {
				 string_arr[i] = prop->prop_data.string_arr.sequence[i];
				 }

			 tango_prop << string_arr;				
			 break;
			 }		

		default:
			Tango::Except::throw_exception 
					((const char *)"Data type error",
	             (const char *)"The requested data type is not implemented for property writing!",
	             (const char *)"c_tango_dbase.c::convert_property_writing()");
			break;
		}
}
