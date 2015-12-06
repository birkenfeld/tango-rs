static const char *RcsId = "$Id: c_tango_attribute.c 25498 2014-04-28 09:03:02Z jensmeyer $\n$Name$";
/******************************************************************************
 * 
 * File       :	c_tango_attribute.c
 *
 * Project    :	C client interface to Tango
 * 
 * Description:	Interface functions to access Tango attributes
 *
 * Original   :	November 2007	
 *	
 * $Author: jensmeyer $
 *
 * $Log$
 * Revision 1.6  2008/03/28 13:34:24  jensmeyer
 * Corrected memory leaks in attribute and command reading.
 *
 * Revision 1.5  2007/12/20 07:57:02  jensmeyer
 * Corrected file headers
 *
 * Revision 1.4  2007/12/18 17:26:20  jensmeyer
 * Added new file for database access and corrected bugs
 *
 * Revision 1.3  2007/12/12 14:20:50  jensmeyer
 * Added doxygen documentation headers and commented code
 *
 * Revision 1.2  2007/12/07 16:05:15  jensmeyer
 * Some name changes to be comaptible with Taco
 *
 * Revision 1.1.1.1  2007/12/05 15:05:04  jensmeyer
 * Tango C language binding
 *
 ******************************************************************************/ 

#include <c_tango.h>
#include <tango.h>

/* internal function definitions */
void translate_exception (Tango::DevFailed& tango_exception, ErrorStack *error);
static void convert_attribute_reading (Tango::DeviceAttribute& devattr, AttributeData *argout);
static void convert_attribute_writing (AttributeData *argin, Tango::DeviceAttribute& devattr);
static void convert_attr_query (Tango::AttributeInfo& tango_attr_info, AttributeInfo *attr_info);

/********************************/
/* External interface functions */
/********************************/

bool tango_read_attributes (void *proxy, VarStringArray *attr_names, AttributeDataList *argout, ErrorStack *error)
{
	vector<Tango::DeviceAttribute> *devattr_list;
	Tango::DeviceProxy *dev;
	
	try
		{
		dev = (Tango::DeviceProxy *) proxy;
		
		/* copy the attribute names to a vecort of string */
		vector<string> names;
		for (int i=0; i<attr_names->length; i++)
			{
			names.push_back(attr_names->sequence[i]);
			}
		
		devattr_list = dev->read_attributes(names);
		
		/* allocate the  AttributeDataList for the number of attributes returned */
		argout->length   = devattr_list->size();
		argout->sequence = new AttributeData[argout->length];
		
		/* loop over all returned attributes and convert the data */
		for (int i=0; i < devattr_list->size(); i++)
			{
			convert_attribute_reading ((*devattr_list)[i], &(argout->sequence[i]));
			}
		
		// The memory is copied, we can now free the returned data
		delete devattr_list;
		}
	
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);		
		return false;
		}
			
	return true;
}


/*@{*/
/**
 * Read an attributes from a Tango device
 *
 * @param proxy Pointer to the device proxy to access the device
 * @param attr_name The name of the attributesto be read
 * @param argout An AttributeData structure with the reading result
 * @param error A pointer to an error stack which will be initialised in case of
 * an execution error
 *
 * @return A boolean set to false when an error occured during execution
 */
bool tango_read_attribute (void *proxy, char *attr_name, AttributeData *argout, ErrorStack *error)
{
	Tango::DeviceAttribute devattr;
	Tango::DeviceProxy *dev;
	
	try
		{
		dev = (Tango::DeviceProxy *) proxy;
		devattr = dev->read_attribute(attr_name);
		
		convert_attribute_reading (devattr, argout);
		}
	
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);		
		return false;
		}	
	
	return true;
}


/*@{*/
/**
 * Write to a list of attributes of a Tango device
 *
 * @param proxy Pointer to the device proxy to access the device
 * @param attr_names The list of attributes to write to
 * @param argin A sequence of AttributeData structures with the data to be written
 * @param error A pointer to an error stack which will be initialised in case of
 * an execution error
 *
 * @return A boolean set to false when an error occured during execution
 */
bool tango_write_attributes (void *proxy, AttributeDataList *argin, ErrorStack *error)
{
	vector<Tango::DeviceAttribute> devattr_list (argin->length);
	Tango::DeviceProxy *dev;
	
	try
		{
		dev = (Tango::DeviceProxy *) proxy;
		
		for (int i=0; i<argin->length; i++)
			{
			convert_attribute_writing (&(argin->sequence[i]), devattr_list[i]);
			}
		
		dev->write_attributes (devattr_list);
		}
	
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);		
		return false;
		}	
				
	return true;
}


/*@{*/
/**
 * Write to an attributes of a Tango device
 *
 * @param proxy Pointer to the device proxy to access the device
 * @param argin An AttributeData structure with the data to be written
 * @param error A pointer to an error stack which will be initialised in case of
 * an execution error
 *
 * @return A boolean set to false when an error occured during execution
 */
bool tango_write_attribute (void *proxy, AttributeData *argin, ErrorStack *error)
{
	Tango::DeviceAttribute devattr;
	Tango::DeviceProxy *dev;
	
	try
		{
		dev = (Tango::DeviceProxy *) proxy;
		
		convert_attribute_writing (argin, devattr);
		dev->write_attribute (devattr);
		}
	
	catch (Tango::DevFailed &tango_exception)
		{
		translate_exception (tango_exception, error);		
		return false;
		}
					
	return true;
}

/* functions to free allocated memory */

/*@{*/
/**
 * Free allocated attribute data.
 * The data was allocated when using the tango_read_attribute() function.
 *
 * @param attribute_data A pointer to the allocated AttributeData structure.
 */
void tango_free_AttributeData (AttributeData *attribute_data)
{
	delete[] (attribute_data->name);
	attribute_data->name = NULL;
	
	switch (attribute_data->data_type)
		{
		case DEV_BOOLEAN:
			if ( attribute_data->attr_data.bool_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.bool_arr.sequence);
			
			attribute_data->attr_data.bool_arr.sequence = NULL;
			attribute_data->attr_data.bool_arr.length = 0;
			break;
		
		case DEV_UCHAR:
			if ( attribute_data->attr_data.char_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.char_arr.sequence);
			
			attribute_data->attr_data.char_arr.sequence = NULL;
			attribute_data->attr_data.char_arr.length = 0;
			break;
											
		case DEV_SHORT:
			if ( attribute_data->attr_data.short_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.short_arr.sequence);
			
			attribute_data->attr_data.short_arr.sequence = NULL;
			attribute_data->attr_data.short_arr.length = 0;
			break;
	
		case DEV_USHORT:
			if ( attribute_data->attr_data.ushort_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.ushort_arr.sequence);
			
			attribute_data->attr_data.ushort_arr.sequence = NULL;
			attribute_data->attr_data.ushort_arr.length = 0;
			break;	
			
		case DEV_LONG:
			if ( attribute_data->attr_data.long_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.long_arr.sequence);
			
			attribute_data->attr_data.long_arr.sequence = NULL;
			attribute_data->attr_data.long_arr.length = 0;
			break;			
	
		case DEV_ULONG:
			if ( attribute_data->attr_data.ulong_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.ulong_arr.sequence);
			
			attribute_data->attr_data.ulong_arr.sequence = NULL;
			attribute_data->attr_data.ulong_arr.length = 0;
			break;	

		case DEV_LONG64:
			if ( attribute_data->attr_data.long64_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.long64_arr.sequence);
			
			attribute_data->attr_data.long64_arr.sequence = NULL;
			attribute_data->attr_data.long64_arr.length = 0;
			break;			
	
		case DEV_ULONG64:
			if ( attribute_data->attr_data.ulong64_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.ulong64_arr.sequence);
			
			attribute_data->attr_data.ulong64_arr.sequence = NULL;
			attribute_data->attr_data.ulong64_arr.length = 0;
			break;
	
		case DEV_FLOAT:
			if ( attribute_data->attr_data.float_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.float_arr.sequence);
			
			attribute_data->attr_data.float_arr.sequence = NULL;
			attribute_data->attr_data.float_arr.length = 0;
			break;	
					
		case DEV_DOUBLE:
			if ( attribute_data->attr_data.double_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.double_arr.sequence);
			
			attribute_data->attr_data.double_arr.sequence = NULL;
			attribute_data->attr_data.double_arr.length = 0;
			break;
			
		case DEV_STATE:
			if ( attribute_data->attr_data.state_arr.sequence != NULL )
				delete[] (attribute_data->attr_data.state_arr.sequence);
			
			attribute_data->attr_data.state_arr.sequence = NULL;
			attribute_data->attr_data.state_arr.length = 0;
			break;
			
		case DEV_STRING:
			for (int i=0; i<attribute_data->attr_data.string_arr.length; i++ )
				{
				delete[] (attribute_data->attr_data.string_arr.sequence[i]);
				}
				
			if ( attribute_data->attr_data.string_arr.sequence != NULL )
				free (attribute_data->attr_data.string_arr.sequence);
			
			attribute_data->attr_data.string_arr.sequence = NULL;
			attribute_data->attr_data.string_arr.length = 0;
			break;											
		
		case DEV_ENCODED:
			for (int i=0; i<attribute_data->attr_data.encoded_arr.length; i++ )
				{
				delete[] (attribute_data->attr_data.encoded_arr.sequence[i].encoded_format);
				free (attribute_data->attr_data.encoded_arr.sequence[i].encoded_data);
				}
				
			if ( attribute_data->attr_data.encoded_arr.sequence != NULL )
				free (attribute_data->attr_data.encoded_arr.sequence);
			
			attribute_data->attr_data.encoded_arr.sequence = NULL;
			attribute_data->attr_data.encoded_arr.length = 0;
			break;
		}		
}


/*@{*/
/**
 * Free an allocated attribute data list.
 * The data list was allocated when using the tango_read_attributes() function.
 *
 * @param attribute_data_list A pointer to the allocated AttributeDataList structure.
 */
void tango_free_AttributeDataList (AttributeDataList *attribute_data_list)
{
	for (int i=0; i<attribute_data_list->length; i++)
		{
		tango_free_AttributeData ( &(attribute_data_list->sequence[i]) );
		}	
	
	delete[] (attribute_data_list->sequence);
	attribute_data_list->sequence = NULL;
	attribute_data_list->length   = 0;		
}




bool tango_get_attribute_list (void *proxy, VarStringArray *attr_names, ErrorStack *error)
{
	Tango::DeviceProxy *dev;
	vector<string>	*attr_list;
	int nb_data;
	
	try
		{
		dev = (Tango::DeviceProxy *) proxy;
	
		attr_list = dev->get_attribute_list();
		nb_data   = attr_list->size();

		/* allocate sequence */
		attr_names->sequence = (char **) calloc(nb_data, sizeof(char *));
		attr_names->length   = nb_data;

		/* allocate strings and copy data */
		for (int i=0 ; i<nb_data ; i++)
			{
			attr_names->sequence[i] = new char[(*attr_list)[i].length() + 1];
			sprintf (attr_names->sequence[i], "%s", (*attr_list)[i].c_str());
			}			
		delete (attr_list);
		}
	
	catch (Tango::DevFailed &tango_exception)
		{
		delete (attr_list);
		translate_exception (tango_exception, error);		
		return false;
		}

	return true;
}


bool tango_get_attribute_config (void *proxy, VarStringArray *attr_names, AttributeInfoList *attr_info_list, ErrorStack *error)
{
	Tango::DeviceProxy *dev;
	vector<Tango::AttributeInfo> *tango_attr_info_list;
	
	try
		{
		dev = (Tango::DeviceProxy *) proxy;
		
		/* copy the attribute names to a vecort of string */
		vector<string> names;
		for (int i=0; i<attr_names->length; i++)
			{
			names.push_back(attr_names->sequence[i]);
			}
				
		tango_attr_info_list = dev->get_attribute_config(names);

		/* allocate the  AttributeInfoList for the number of attributes returned */
		attr_info_list->length   = tango_attr_info_list->size();
		attr_info_list->sequence = new AttributeInfo[attr_info_list->length];
		
		/* loop over all returned attributes and convert the data */
		for (int i=0; i < tango_attr_info_list->size(); i++)
			{
			convert_attr_query ((*tango_attr_info_list)[i], &(attr_info_list->sequence[i]));
			}
		delete (tango_attr_info_list);
		}
	
	catch (Tango::DevFailed &tango_exception)
		{
		delete (tango_attr_info_list);
		translate_exception (tango_exception, error);		
		return false;
		}


	return true;
}


bool tango_attribute_list_query (void *proxy, AttributeInfoList *attr_info_list, ErrorStack *error)
{
	Tango::DeviceProxy *dev;
	vector<Tango::AttributeInfo> *tango_attr_info_list;
	
	try
		{
		dev = (Tango::DeviceProxy *) proxy;
		tango_attr_info_list = dev->attribute_list_query();

		/* allocate the  AttributeInfoList for the number of attributes returned */
		attr_info_list->length   = tango_attr_info_list->size();
		attr_info_list->sequence = new AttributeInfo[attr_info_list->length];
		
		/* loop over all returned attributes and convert the data */
		for (int i=0; i < tango_attr_info_list->size(); i++)
			{
			convert_attr_query ((*tango_attr_info_list)[i], &(attr_info_list->sequence[i]));
			}
		delete (tango_attr_info_list);
		}
	
	catch (Tango::DevFailed &tango_exception)
		{
		delete (tango_attr_info_list);
		translate_exception (tango_exception, error);		
		return false;
		}

	return true;
}

void tango_free_VarStringArray (VarStringArray *string_arr)
{
	for (int i=0; i<string_arr->length; i++)
		{
		delete[] (string_arr->sequence[i]);
		}	
	
	free (string_arr->sequence);
	string_arr->sequence = NULL;
	string_arr->length   = 0;
}


void tango_free_AttributeInfoList (AttributeInfoList 	*attribute_info_list)
{
	for (int i=0; i<attribute_info_list->length; i++)
		{
		 delete[] (attribute_info_list->sequence[i].name);
		 delete[] (attribute_info_list->sequence[i].description);
		 delete[] (attribute_info_list->sequence[i].label);
		 delete[] (attribute_info_list->sequence[i].unit);
		 delete[] (attribute_info_list->sequence[i].standard_unit);
		 delete[] (attribute_info_list->sequence[i].display_unit);
		 delete[] (attribute_info_list->sequence[i].format);
		 delete[] (attribute_info_list->sequence[i].min_value);
		 delete[] (attribute_info_list->sequence[i].max_value);
		 delete[] (attribute_info_list->sequence[i].min_alarm);
		 delete[] (attribute_info_list->sequence[i].max_alarm);
		 delete[] (attribute_info_list->sequence[i].writable_attr_name);
		}	
	
	delete[] (attribute_info_list->sequence);
	attribute_info_list->sequence = NULL;
	attribute_info_list->length   = 0;
}

/*@}*/


/*************************************/
/* internal library helper functions */
/*************************************/


void convert_attribute_reading (Tango::DeviceAttribute& devattr, AttributeData *argout)
{
	/* treat INVALID data quality */
	if (devattr.get_quality() == Tango::ATTR_INVALID )
	{
		/* Just initialise the first datatype. This should be valid for
		   all data types in the union! */
		   
		argout->attr_data.bool_arr.length   = 0;
		argout->attr_data.bool_arr.sequence = NULL;
	}
	else
	{
	
	/* get data type */
	argout->data_type = (TangoDataType) devattr.get_type();
	argout->data_format = (AttrDataFormat) devattr.data_format;
	
	switch (argout->data_type)
		{
		case DEV_BOOLEAN:
			{
			Tango::DevVarBooleanArray *bool_seq;
			int			               nb_data;
				
			devattr >> bool_seq;
			nb_data =  bool_seq->length();

			argout->attr_data.bool_arr.length   = nb_data;	
			argout->attr_data.bool_arr.sequence = new bool[nb_data];

			memcpy ( argout->attr_data.bool_arr.sequence,
				     bool_seq->get_buffer(), 
					(sizeof(bool) * nb_data) );			  
			
			delete bool_seq;
			break;
			}

		case DEV_UCHAR:
			{
			Tango::DevVarCharArray *char_seq;
			int			               nb_data;

			devattr >> char_seq;
			nb_data =  char_seq->length();

			argout->attr_data.char_arr.length   = nb_data;	
			argout->attr_data.char_arr.sequence = new unsigned char[nb_data];

			memcpy ( argout->attr_data.char_arr.sequence,
				     char_seq->get_buffer(), 
					(sizeof(unsigned char) * nb_data) );			  
			
			delete char_seq;
			break;
			}

		case DEV_SHORT:
			{
			Tango::DevVarShortArray *short_seq;
			int			               nb_data;

			devattr >> short_seq;
			nb_data =  short_seq->length();

			argout->attr_data.short_arr.length   = nb_data;	
			argout->attr_data.short_arr.sequence = new short[nb_data];

			memcpy ( argout->attr_data.short_arr.sequence,
				     short_seq->get_buffer(), 
					(sizeof(short) * nb_data) );			  
			
			delete short_seq;
			break;
			}
			
		case DEV_USHORT:
			{
			Tango::DevVarUShortArray *ushort_seq;
			int			               nb_data;

			devattr >> ushort_seq;
			nb_data =  ushort_seq->length();

			argout->attr_data.ushort_arr.length   = nb_data;	
			argout->attr_data.ushort_arr.sequence = new unsigned short[nb_data];

			memcpy ( argout->attr_data.ushort_arr.sequence,
				     ushort_seq->get_buffer(), 
					(sizeof(unsigned short) * nb_data) );			  
			
			delete ushort_seq;
			break;
			}			
			
		case DEV_LONG:
			{
			Tango::DevVarLongArray *long_seq;
			int			               nb_data;

			devattr >> long_seq;
			nb_data =  long_seq->length();

			argout->attr_data.long_arr.length   = nb_data;	
			argout->attr_data.long_arr.sequence = new int[nb_data];

			memcpy ( argout->attr_data.long_arr.sequence,
				     long_seq->get_buffer(), 
					(sizeof(int) * nb_data) );			  
			
			delete long_seq;
			break;
			}	
			
		case DEV_ULONG:
			{
			Tango::DevVarULongArray *ulong_seq;
			int			               nb_data;

			devattr >> ulong_seq;
			nb_data =  ulong_seq->length();

			argout->attr_data.ulong_arr.length   = nb_data;	
			argout->attr_data.ulong_arr.sequence = new unsigned int[nb_data];

			memcpy ( argout->attr_data.ulong_arr.sequence,
				     ulong_seq->get_buffer(), 
					(sizeof(unsigned int) * nb_data) );			  
			
			delete ulong_seq;
			break;
			}				
	
		case DEV_LONG64:
			{
			Tango::DevVarLong64Array *long64_seq;
			int			               nb_data;

			devattr >> long64_seq;
			nb_data =  long64_seq->length();

			argout->attr_data.long64_arr.length   = nb_data;			
			argout->attr_data.long64_arr.sequence = new Tango::DevLong64[nb_data];

			memcpy ( argout->attr_data.long64_arr.sequence,
				     long64_seq->get_buffer(), 
					(sizeof(Tango::DevLong64) * nb_data) );									  
			
			delete long64_seq;
			break;
			}	
			
		case DEV_ULONG64:
			{
			Tango::DevVarULong64Array *ulong64_seq;
			int			               nb_data;

			devattr >> ulong64_seq;
			nb_data =  ulong64_seq->length();

			argout->attr_data.ulong64_arr.length   = nb_data;
			argout->attr_data.ulong64_arr.sequence = new Tango::DevULong64[nb_data];

			memcpy ( argout->attr_data.ulong64_arr.sequence,
				     ulong64_seq->get_buffer(), 
					(sizeof(Tango::DevULong64) * nb_data) );
			
			delete ulong64_seq;
			break;
			}		
									
		case DEV_FLOAT:
			{
			Tango::DevVarFloatArray *float_seq;
			int			               nb_data;

			devattr >> float_seq;
			nb_data =  float_seq->length();

			argout->attr_data.float_arr.length   = nb_data;	
			argout->attr_data.float_arr.sequence = new float[nb_data];

			memcpy ( argout->attr_data.float_arr.sequence,
				     float_seq->get_buffer(), 
					(sizeof(float) * nb_data) );			  
			
			delete float_seq;
			break;
			} 

		case DEV_DOUBLE:
			{
			Tango::DevVarDoubleArray *double_seq;
			int			               nb_data;

			devattr >> double_seq;
			nb_data =  double_seq->length();

			argout->attr_data.double_arr.length   = nb_data;	
			argout->attr_data.double_arr.sequence = new double[nb_data];

			memcpy ( argout->attr_data.double_arr.sequence,
				     double_seq->get_buffer(), 
					(sizeof(double) * nb_data) );			  
			
			delete double_seq;
			break;
			} 
			
			
		case DEV_STRING:
			{
			vector<string>	string_vect;
			int nb_data;

		  
			devattr >> string_vect;
			nb_data = string_vect.size();

			/* allocate sequence */
			argout->attr_data.string_arr.sequence = (char **) calloc(nb_data, sizeof(char *));;
			argout->attr_data.string_arr.length   = nb_data;

			/* allocate strings and copy data */
			for (int i=0 ; i<nb_data ; i++)
				{
				argout->attr_data.string_arr.sequence[i] = new char[string_vect[i].length() + 1];
				sprintf (argout->attr_data.string_arr.sequence[i], "%s", 
				         string_vect[i].c_str());
				}
			break;
			} 			


		case DEV_STATE:
			{
			vector<Tango::DevState>	state_vect;
			int nb_data;

			/* The State attribute is not returning a sequence!!!!
			   Check whether the attribute name is State! */

			if ( devattr.name == "State" )
				{
				state_vect.resize(1);
				devattr >> state_vect[0];
				}
			else
				{		  
				devattr >> state_vect;
				}

			nb_data = state_vect.size();

			/* allocate sequence */
			argout->attr_data.state_arr.sequence = new TangoDevState[nb_data];
			argout->attr_data.state_arr.length   = nb_data;

			/* copy data */
			for (int i=0 ; i<nb_data ; i++)
				{
				argout->attr_data.state_arr.sequence[i] = (TangoDevState) state_vect[i];
				}
			break;
			}
			
			
		case DEV_ENCODED:
			{
			/* vector<Tango::DevEncoded>	encoded_vect; */
			Tango::DevVarEncodedArray *encoded_vect;
			int nb_data;

		  
			devattr >> encoded_vect;
			/* nb_data =  encoded_vect.size(); */
			nb_data =  encoded_vect->length();

			/* allocate sequence */
			argout->attr_data.encoded_arr.sequence = (TangoDevEncoded *) calloc(nb_data, sizeof(TangoDevEncoded *));;
			argout->attr_data.encoded_arr.length   = nb_data;

			/* allocate the encoded structues and copy data */
			for (int i=0 ; i<nb_data ; i++)
				{
				string format ((*encoded_vect)[i].encoded_format);
				argout->attr_data.encoded_arr.sequence[i].encoded_format = new char[format.size() + 1];
				sprintf (argout->attr_data.encoded_arr.sequence[i].encoded_format, "%s", format.c_str());
				
				/* get the pointer to the buffer and take over the memory */
				/* setting the parameter to true, does not free the memory when freeing the CORBA sequence */
				
				argout->attr_data.encoded_arr.sequence[i].encoded_length = 
						(*encoded_vect)[i].encoded_data.length();
				argout->attr_data.encoded_arr.sequence[i].encoded_data   =
						(unsigned char *)(*encoded_vect)[i].encoded_data.get_buffer(true);
								
				}
			break;
			} 			
				
		
		default:
				Tango::Except::throw_exception 
						((const char *)"Data type error",
	                (const char *)"The requested data type is not implemented for attribute reading!",
	                (const char *)"c_tango_attribute.c::convert_attribute_reading()");
				break;		
		}
	}

	/* get quality factor */
	argout->quality = (AttrQuality) devattr.get_quality();

	/* copy timestamp */
	argout->time_stamp.tv_sec  = devattr.time.tv_sec;
	argout->time_stamp.tv_usec = devattr.time.tv_usec;

	/* allocate attribute name */
	argout->name = new char[devattr.name.length() + 1];
	sprintf (argout->name, "%s", devattr.name.c_str());

	/* get data dimension */
	argout->dim_x = devattr.dim_x;
	argout->dim_y = devattr.dim_y;
} 



void convert_attribute_writing (AttributeData *argin, Tango::DeviceAttribute& devattr)
{
		/* allocate a vector and copy the data */
		
		switch (argin->data_type)
			{
			case DEV_BOOLEAN:
				{
				/* copy the data into a boolean vector */
				vector<bool> bool_arr(argin->attr_data.bool_arr.length);
				for (int i=0; i<argin->attr_data.bool_arr.length; i++)
					{
					bool_arr[i] = (bool) argin->attr_data.bool_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (bool_arr, argin->dim_x, argin->dim_y);
				break;
				}
				
			case DEV_UCHAR:
				{
				/* copy the data into a char vector */
				vector< unsigned char> char_arr(argin->attr_data.char_arr.length);
				for (int i=0; i<argin->attr_data.char_arr.length; i++)
					{
					char_arr[i] = (unsigned char) argin->attr_data.char_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (char_arr, argin->dim_x, argin->dim_y);
				break;
				}														
			
			case DEV_SHORT:
				{
				/* copy the data into a short vector */
				vector<short> short_arr(argin->attr_data.short_arr.length);
				for (int i=0; i<argin->attr_data.short_arr.length; i++)
					{
					short_arr[i] = (short) argin->attr_data.short_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (short_arr, argin->dim_x, argin->dim_y);
				break;
				}
				
			case DEV_USHORT:
				{
				/* copy the data into a unsigned short vector */
				vector<unsigned short> ushort_arr(argin->attr_data.ushort_arr.length);
				for (int i=0; i<argin->attr_data.ushort_arr.length; i++)
					{
					ushort_arr[i] = (unsigned short) argin->attr_data.ushort_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (ushort_arr, argin->dim_x, argin->dim_y);
				break;
				}					
				
			case DEV_LONG:
				{
				/* copy the data into a long vector */
				vector<Tango::DevLong> long_arr(argin->attr_data.long_arr.length);
				for (int i=0; i<argin->attr_data.long_arr.length; i++)
					{
					long_arr[i] = (int) argin->attr_data.long_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (long_arr, argin->dim_x, argin->dim_y);
				break;
				}
				
			case DEV_ULONG:
				{
				/* copy the data into a unsigned vector */
				vector<Tango::DevULong> ulong_arr(argin->attr_data.ulong_arr.length);
				for (int i=0; i<argin->attr_data.ulong_arr.length; i++)
					{
					ulong_arr[i] = (unsigned int) argin->attr_data.ulong_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (ulong_arr, argin->dim_x, argin->dim_y);
				break;
				}
				
			case DEV_LONG64:
				{
				/* copy the data into a long64 vector */				
				vector<Tango::DevLong64> long_arr(argin->attr_data.long_arr.length);
			
				for (int i=0; i<argin->attr_data.long_arr.length; i++)
					{
					long_arr[i] = (Tango::DevLong64) argin->attr_data.long_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (long_arr, argin->dim_x, argin->dim_y);
				break;
				}
				
			case DEV_ULONG64:
				{
				/* copy the data into a unsigned long64 vector */
				vector<Tango::DevULong64> ulong64_arr(argin->attr_data.ulong64_arr.length);
				
				for (int i=0; i<argin->attr_data.ulong64_arr.length; i++)
					{
					ulong64_arr[i] = (Tango::DevULong64) argin->attr_data.ulong64_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (ulong64_arr, argin->dim_x, argin->dim_y);
				break;
				}				
														
			case DEV_FLOAT:
				{
				/* copy the data into a double vector */
				vector<float> float_arr(argin->attr_data.float_arr.length);
				for (int i=0; i<argin->attr_data.float_arr.length; i++)
					{
					float_arr[i] = (float) argin->attr_data.float_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (float_arr, argin->dim_x, argin->dim_y);
				break;
				}	
			
			case DEV_DOUBLE:
				{
				/* copy the data into a double vector */
				vector<double> double_arr(argin->attr_data.double_arr.length);
				for (int i=0; i<argin->attr_data.double_arr.length; i++)
					{
					double_arr[i] = (double) argin->attr_data.double_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (double_arr, argin->dim_x, argin->dim_y);
				break;
				}
				
			case DEV_STRING:
				{
				/* copy the data into a string vector */
				vector<string> string_arr(argin->attr_data.string_arr.length);
				
				for (int i=0; i<argin->attr_data.string_arr.length; i++)
					{
					string_arr[i] = argin->attr_data.string_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (string_arr, argin->dim_x, argin->dim_y);
				break;
				}				
				
			case DEV_STATE:
				{
				/* copy the data into a state vector */
				vector<Tango::DevState> state_arr(argin->attr_data.state_arr.length);
				for (int i=0; i<argin->attr_data.state_arr.length; i++)
					{
					state_arr[i] = (Tango::DevState) argin->attr_data.state_arr.sequence[i];
					}
				
				/* Inert into the DeviceAtrribute object */
				devattr.insert (state_arr, argin->dim_x, argin->dim_y);
				break;
				}
				
			case DEV_ENCODED:
				{
				/* today encoded type is only avalable as SCALAR data type */
				
				/* Insert into the DeviceAtrribute object */
				devattr.insert (argin->attr_data.encoded_arr.sequence[0].encoded_format, 
				                argin->attr_data.encoded_arr.sequence[0].encoded_data,
								argin->attr_data.encoded_arr.sequence[0].encoded_length);				
				break;
				}
				
			default:
				Tango::Except::throw_exception 
					((const char *)"Data type error",
	                (const char *)"The requested data type is not implemented for attribute writing!",
	                (const char *)"c_tango_attribute.c::convert_attribute_writing()");
				break;									
			}		
		
		/* set attribute name */
		devattr.set_name(argin->name);
} 



static void convert_attr_query (Tango::AttributeInfo& tango_attr_info, AttributeInfo *attr_info)
{
	/* allocate attribute name */
	attr_info->name = new char[tango_attr_info.name.length() + 1];
	sprintf (attr_info->name, "%s", tango_attr_info.name.c_str());

	attr_info->description = new char[tango_attr_info.description.length() + 1];
	sprintf (attr_info->description, "%s", tango_attr_info.description.c_str());

	attr_info->label = new char[tango_attr_info.label.length() + 1];
	sprintf (attr_info->label, "%s", tango_attr_info.label.c_str());
	
	attr_info->unit = new char[tango_attr_info.unit.length() + 1];
	sprintf (attr_info->unit, "%s", tango_attr_info.unit.c_str());
	
	attr_info->standard_unit = new char[tango_attr_info.standard_unit.length() + 1];
	sprintf (attr_info->standard_unit, "%s", tango_attr_info.standard_unit.c_str());
	
	attr_info->display_unit = new char[tango_attr_info.display_unit.length() + 1];
	sprintf (attr_info->display_unit, "%s", tango_attr_info.display_unit.c_str());
	
	attr_info->format = new char[tango_attr_info.format.length() + 1];
	sprintf (attr_info->format, "%s", tango_attr_info.format.c_str());
	
	attr_info->min_value = new char[tango_attr_info.min_value.length() + 1];
	sprintf (attr_info->min_value, "%s", tango_attr_info.min_value.c_str());
	
	attr_info->max_value = new char[tango_attr_info.max_value.length() + 1];
	sprintf (attr_info->max_value, "%s", tango_attr_info.max_value.c_str());
	
	attr_info->min_alarm = new char[tango_attr_info.min_alarm.length() + 1];
	sprintf (attr_info->min_alarm, "%s", tango_attr_info.min_alarm.c_str());
	
	attr_info->max_alarm = new char[tango_attr_info.max_alarm.length() + 1];
	sprintf (attr_info->max_alarm, "%s", tango_attr_info.max_alarm.c_str());
	
	attr_info->writable_attr_name = new char[tango_attr_info.writable_attr_name.length() + 1];
	sprintf (attr_info->writable_attr_name, "%s", tango_attr_info.writable_attr_name.c_str());
	
	attr_info->writable    = (AttrWriteType) tango_attr_info.writable;
	attr_info->data_format = (AttrDataFormat) tango_attr_info.data_format;
	attr_info->data_type   = (TangoDataType) tango_attr_info.data_type;
	attr_info->max_dim_x   = tango_attr_info.max_dim_x;
	attr_info->max_dim_y   = tango_attr_info.max_dim_y;	
	
	attr_info->disp_level = (DispLevel) tango_attr_info.disp_level;
}
