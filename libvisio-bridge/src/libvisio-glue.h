#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>

typedef struct visio_glue_input_stream_s visio_glue_input_stream;
typedef struct visio_glue_painter_s visio_glue_painter;
typedef struct visio_glue_property_list_s visio_glue_property_list;
typedef struct visio_glue_property_list_iterator_s visio_glue_property_list_iterator;

typedef void (*visio_glue_painter_func)(void *user_ptr);
typedef void (*visio_glue_painter_func_pl)(void *user_ptr, const visio_glue_property_list *prop_list);
typedef void (*visio_glue_painter_func_str)(void *user_ptr, const char *text, size_t length);

typedef struct visio_glue_painter_funcs_s {
    visio_glue_painter_func destroy;

    visio_glue_painter_func_pl start_document;
    visio_glue_painter_func end_document;
    visio_glue_painter_func_pl start_page;
    visio_glue_painter_func end_page;
    visio_glue_painter_func_pl start_master_page;
    visio_glue_painter_func end_master_page;
    visio_glue_painter_func_pl start_layer;
    visio_glue_painter_func end_layer;
    visio_glue_painter_func_pl start_embedded_graphics;
    visio_glue_painter_func end_embedded_graphics;
    visio_glue_painter_func_pl open_group;
    visio_glue_painter_func close_group;
    visio_glue_painter_func_pl start_text_object;
    visio_glue_painter_func end_text_object;
    visio_glue_painter_func_pl start_table_object;
    visio_glue_painter_func end_table_object;
    visio_glue_painter_func_pl open_table_row;
    visio_glue_painter_func close_table_row;
    visio_glue_painter_func_pl open_table_cell;
    visio_glue_painter_func close_table_cell;
    visio_glue_painter_func_pl open_ordered_list_level;
    visio_glue_painter_func close_ordered_list_level;
    visio_glue_painter_func_pl open_unordered_list_level;
    visio_glue_painter_func close_unordered_list_level;
    visio_glue_painter_func_pl open_list_element;
    visio_glue_painter_func close_list_element;
    visio_glue_painter_func_pl open_paragraph;
    visio_glue_painter_func close_paragraph;
    visio_glue_painter_func_pl open_span;
    visio_glue_painter_func close_span;
    visio_glue_painter_func_pl open_link;
    visio_glue_painter_func close_link;

    visio_glue_painter_func_pl set_document_metadata;
    visio_glue_painter_func_pl define_embedded_font;
    visio_glue_painter_func_pl set_style;
    visio_glue_painter_func_pl draw_rectangle;
    visio_glue_painter_func_pl draw_ellipse;
    visio_glue_painter_func_pl draw_polygon;
    visio_glue_painter_func_pl draw_polyline;
    visio_glue_painter_func_pl draw_path;
    visio_glue_painter_func_pl draw_graphic_object;
    visio_glue_painter_func_pl draw_connector;
    visio_glue_painter_func_pl insert_covered_table_cell;
    visio_glue_painter_func_pl insert_field;
    visio_glue_painter_func_pl define_paragraph_style;
    visio_glue_painter_func_pl define_character_style;

    visio_glue_painter_func insert_tab;
    visio_glue_painter_func insert_space;
    visio_glue_painter_func insert_line_break;

    visio_glue_painter_func_str insert_text;
} visio_glue_painter_funcs;

typedef enum visio_glue_seek_type_e {
    VISIO_GLUE_SEEK_CUR,
    VISIO_GLUE_SEEK_START,
    VISIO_GLUE_SEEK_END
} visio_glue_seek_type;

typedef struct visio_glue_input_stream_funcs_s {
    void (*destroy)(void *user_ptr);

    bool (*is_structured)(void *user_ptr);
    unsigned int (*sub_stream_count)(void *user_ptr);
    const char *(*sub_stream_name)(void *user_ptr, unsigned int stream_id);
    bool (*sub_stream_exists)(void *user_ptr, const char *name);
    struct visio_glue_input_stream_s *(*sub_stream_by_id)(void *user_ptr, unsigned int stream_id);
    struct visio_glue_input_stream_s *(*sub_stream_by_name)(void *user_ptr, const char *name);
    const unsigned char *(*read)(void *user_ptr, unsigned long num_bytes, unsigned long *num_bytes_read);
    int (*seek)(void *user_ptr, long offset, visio_glue_seek_type seek_type);
    long (*tell)(void *user_ptr);
    bool (*is_end)(void *user_ptr);
} visio_glue_input_stream_funcs;

typedef struct visio_glue_property_value_s {
    char *value;
} visio_glue_property_value;

visio_glue_input_stream *visio_glue_new_input_stream(visio_glue_input_stream_funcs funcs, void *user_ptr);
visio_glue_painter *visio_glue_new_painter(visio_glue_painter_funcs funcs, void *user_ptr);

void visio_glue_input_stream_free(visio_glue_input_stream *stream);
void visio_glue_painter_free(visio_glue_painter *painter);

bool visio_glue_document_is_supported(visio_glue_input_stream *stream);
bool visio_glue_document_parse(visio_glue_input_stream *stream, visio_glue_painter *painter);
bool visio_glue_document_parse_stencils(visio_glue_input_stream *stream, visio_glue_painter *painter);

visio_glue_property_list_iterator *visio_glue_property_list_iterate(const visio_glue_property_list *list);
void visio_glue_property_list_iterator_free(visio_glue_property_list_iterator *iterator);
bool visio_glue_property_list_iterator_advance(visio_glue_property_list_iterator *iterator);
const char *visio_glue_property_list_iterator_key(visio_glue_property_list_iterator *iterator);
visio_glue_property_value visio_glue_property_list_iterator_value(visio_glue_property_list_iterator *iterator);
void visio_glue_property_value_free(visio_glue_property_value *value);

#ifdef __cplusplus
}
#endif
