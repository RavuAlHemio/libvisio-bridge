#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>

typedef struct visio_glue_input_stream_s visio_glue_input_stream;
typedef struct visio_glue_property_list_s visio_glue_property_list;
typedef struct visio_glue_property_list_iterator_s visio_glue_property_list_iterator;

#define VG_FUNC(name) void (* name )(void *user_ptr)
#define VG_PLFUNC(name) void (* name )(void *user_ptr, const visio_glue_property_list *prop_list)
#define VG_START_END(start_name, end_name) VG_PLFUNC(start_name); VG_FUNC(end_name)

typedef void (*visio_glue_func)(void *user_ptr);
typedef void (*visio_glue_func_pl)(void *user_ptr, const visio_glue_property_list *prop_list);
typedef void (*visio_glue_func_str)(void *user_ptr, const char *text, size_t length);

typedef struct visio_glue_painter_s {
    visio_glue_func_pl start_document;
    visio_glue_func end_document;
    visio_glue_func_pl start_page;
    visio_glue_func end_page;
    visio_glue_func_pl start_master_page;
    visio_glue_func end_master_page;
    visio_glue_func_pl start_layer;
    visio_glue_func end_layer;
    visio_glue_func_pl start_embedded_graphics;
    visio_glue_func end_embedded_graphics;
    visio_glue_func_pl open_group;
    visio_glue_func close_group;
    visio_glue_func_pl start_text_object;
    visio_glue_func end_text_object;
    visio_glue_func_pl start_table_object;
    visio_glue_func end_table_object;
    visio_glue_func_pl open_table_row;
    visio_glue_func close_table_row;
    visio_glue_func_pl open_table_cell;
    visio_glue_func close_table_cell;
    visio_glue_func_pl open_ordered_list_level;
    visio_glue_func close_ordered_list_level;
    visio_glue_func_pl open_unordered_list_level;
    visio_glue_func close_unordered_list_level;
    visio_glue_func_pl open_list_element;
    visio_glue_func close_list_element;
    visio_glue_func_pl open_paragraph;
    visio_glue_func close_paragraph;
    visio_glue_func_pl open_span;
    visio_glue_func close_span;
    visio_glue_func_pl open_link;
    visio_glue_func close_link;

    visio_glue_func_pl set_document_metadata;
    visio_glue_func_pl define_embedded_font;
    visio_glue_func_pl set_style;
    visio_glue_func_pl draw_rectangle;
    visio_glue_func_pl draw_ellipse;
    visio_glue_func_pl draw_polygon;
    visio_glue_func_pl draw_polyline;
    visio_glue_func_pl draw_path;
    visio_glue_func_pl draw_graphic_object;
    visio_glue_func_pl draw_connector;
    visio_glue_func_pl insert_covered_table_cell;
    visio_glue_func_pl insert_field;
    visio_glue_func_pl define_paragraph_style;
    visio_glue_func_pl define_character_style;

    visio_glue_func insert_tab;
    visio_glue_func insert_space;
    visio_glue_func insert_line_break;

    visio_glue_func_str insert_text;
} visio_glue_painter;

typedef struct visio_glue_property_value_s {
    char *value;
} visio_glue_property_value;

visio_glue_input_stream *visio_glue_open_file(const char *path);
void visio_glue_close_file(visio_glue_input_stream *stream);

bool visio_glue_document_is_supported(visio_glue_input_stream *stream);
bool visio_glue_document_parse(visio_glue_input_stream *stream, visio_glue_painter *painter, void *user_ptr);
bool visio_glue_document_parse_stencils(visio_glue_input_stream *stream, visio_glue_painter *painter, void *user_ptr);

visio_glue_property_list_iterator *visio_glue_property_list_iterate(const visio_glue_property_list *list);
void visio_glue_property_list_iterator_free(visio_glue_property_list_iterator *iterator);
bool visio_glue_property_list_iterator_advance(visio_glue_property_list_iterator *iterator);
const char *visio_glue_property_list_iterator_key(visio_glue_property_list_iterator *iterator);
visio_glue_property_value visio_glue_property_list_iterator_value(visio_glue_property_list_iterator *iterator);
void visio_glue_property_value_free(visio_glue_property_value *value);

#ifdef __cplusplus
}
#endif
