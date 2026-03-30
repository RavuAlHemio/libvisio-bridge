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

typedef struct visio_glue_painter_s {
    VG_START_END(start_document, end_document);
    VG_START_END(start_page, end_page);
    VG_START_END(start_master_page, end_master_page);
    VG_START_END(start_layer, end_layer);
    VG_START_END(start_embedded_graphics, end_embedded_graphics);
    VG_START_END(open_group, close_group);
    VG_START_END(start_text_object, end_text_object);
    VG_START_END(start_table_object, end_table_object);
    VG_START_END(open_table_row, close_table_row);
    VG_START_END(open_table_cell, close_table_cell);
    VG_START_END(open_ordered_list_level, close_ordered_list_level);
    VG_START_END(open_unordered_list_level, close_unordered_list_level);
    VG_START_END(open_list_element, close_list_element);
    VG_START_END(open_paragraph, close_paragraph);
    VG_START_END(open_span, close_span);
    VG_START_END(open_link, close_link);
    VG_PLFUNC(set_document_metadata);
    VG_PLFUNC(define_embedded_font);
    VG_PLFUNC(set_style);
    VG_PLFUNC(draw_rectangle);
    VG_PLFUNC(draw_ellipse);
    VG_PLFUNC(draw_polygon);
    VG_PLFUNC(draw_polyline);
    VG_PLFUNC(draw_path);
    VG_PLFUNC(draw_graphic_object);
    VG_PLFUNC(draw_connector);
    VG_PLFUNC(insert_covered_table_cell);
    VG_PLFUNC(insert_field);
    VG_PLFUNC(define_paragraph_style);
    VG_PLFUNC(define_character_style);
    VG_FUNC(insert_tab);
    VG_FUNC(insert_space);
    VG_FUNC(insert_line_break);
    void (*insert_text)(void *user_ptr, const char *text, size_t length);
} visio_glue_painter;

visio_glue_input_stream *visio_glue_open_file(const char *path);
void visio_glue_close_file(visio_glue_input_stream *stream);

bool visio_glue_document_is_supported(visio_glue_input_stream *stream);
bool visio_glue_document_parse(visio_glue_input_stream *stream, visio_glue_painter *painter, void *user_ptr);
bool visio_glue_document_parse_stencils(visio_glue_input_stream *stream, visio_glue_painter *painter, void *user_ptr);

visio_glue_property_list_iterator *visio_glue_property_list_iterate(visio_glue_property_list *list);
void visio_glue_property_list_iterator_free(visio_glue_property_list_iterator *iterator);
bool visio_glue_property_list_iterator_advance(visio_glue_property_list_iterator *iterator);
const char *visio_glue_property_list_iterator_key(visio_glue_property_list_iterator *iterator);
char *visio_glue_property_list_iterator_value(visio_glue_property_list_iterator *iterator); /* free using free() */

#ifdef __cplusplus
}
#endif
