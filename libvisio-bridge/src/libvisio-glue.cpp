#include <cstdlib>
#include <cstring>

#include <libvisio/libvisio.h>
#include <librevenge/librevenge.h>
#include <librevenge-stream/librevenge-stream.h>

#include "libvisio-glue.h"


using librevenge::RVNGDrawingInterface;
using librevenge::RVNGFileStream;
using librevenge::RVNGInputStream;
using librevenge::RVNGProperty;
using librevenge::RVNGPropertyList;
using librevenge::RVNGString;
using libvisio::VisioDocument;


#define VG_IMPL(method_name, func_name) \
    virtual void method_name() { \
        if (painter_interface != nullptr && painter_interface->func_name != nullptr) { \
            (*painter_interface->func_name)(user_ptr); \
        } \
    }

#define VG_IMPL_PL(method_name, func_name) \
    virtual void method_name(const RVNGPropertyList &propList) { \
        if (painter_interface != nullptr && painter_interface->func_name != nullptr) { \
            (*painter_interface->func_name)(user_ptr, reinterpret_cast<const visio_glue_property_list*>(&propList)); \
        } \
    }

#define VG_IMPL_START_END(start_method, start_func, end_method, end_func) \
    VG_IMPL_PL(start_method, start_func) \
    VG_IMPL(end_method, end_func)


class CDrawingInterface : public RVNGDrawingInterface {
protected:
    visio_glue_painter *painter_interface;
    void *user_ptr;

public:
    CDrawingInterface(visio_glue_painter *painter, void *user_pointer)
        : painter_interface(painter), user_ptr(user_pointer) {
    }

    VG_IMPL_START_END(startDocument, start_document, endDocument, end_document);
    VG_IMPL_PL(setDocumentMetaData, set_document_metadata);
    VG_IMPL_PL(defineEmbeddedFont, define_embedded_font);
    VG_IMPL_START_END(startPage, start_page, endPage, end_page);
    VG_IMPL_START_END(startMasterPage, start_master_page, endMasterPage, end_master_page);
    VG_IMPL_PL(setStyle, set_style);
    VG_IMPL_START_END(startLayer, start_layer, endLayer, end_layer);
    VG_IMPL_START_END(startEmbeddedGraphics, start_embedded_graphics, endEmbeddedGraphics, end_embedded_graphics);
    VG_IMPL_START_END(openGroup, open_group, closeGroup, close_group);
    VG_IMPL_PL(drawRectangle, draw_rectangle);
    VG_IMPL_PL(drawEllipse, draw_ellipse);
    VG_IMPL_PL(drawPolygon, draw_polygon);
    VG_IMPL_PL(drawPolyline, draw_polyline);
    VG_IMPL_PL(drawPath, draw_path);
    VG_IMPL_PL(drawGraphicObject, draw_graphic_object);
    VG_IMPL_PL(drawConnector, draw_connector);
    VG_IMPL_START_END(startTextObject, start_text_object, endTextObject, end_text_object);
    VG_IMPL_START_END(startTableObject, start_table_object, endTableObject, end_table_object);
    VG_IMPL_START_END(openTableRow, open_table_row, closeTableRow, close_table_row);
    VG_IMPL_START_END(openTableCell, open_table_cell, closeTableCell, close_table_cell);
    VG_IMPL_PL(insertCoveredTableCell, insert_covered_table_cell);
    VG_IMPL(insertTab, insert_tab);
    VG_IMPL(insertSpace, insert_space);

    virtual void insertText(const RVNGString &text) {
        if (painter_interface != nullptr && painter_interface->insert_text != nullptr) {
            (*painter_interface->insert_text)(user_ptr, text.cstr(), text.len());
        }
    }

    VG_IMPL(insertLineBreak, insert_line_break);
    VG_IMPL_PL(insertField, insert_field);
    VG_IMPL_START_END(openOrderedListLevel, open_ordered_list_level, closeOrderedListLevel, close_ordered_list_level);
    VG_IMPL_START_END(openUnorderedListLevel, open_unordered_list_level, closeUnorderedListLevel, close_unordered_list_level);
    VG_IMPL_START_END(openListElement, open_list_element, closeListElement, close_list_element);
    VG_IMPL_PL(defineParagraphStyle, define_paragraph_style);
    VG_IMPL_START_END(openParagraph, open_paragraph, closeParagraph, close_paragraph);
    VG_IMPL_PL(defineCharacterStyle, define_character_style);
    VG_IMPL_START_END(openSpan, open_span, closeSpan, close_span);
    VG_IMPL_START_END(openLink, open_link, closeLink, close_link);
};


extern "C" visio_glue_input_stream *
visio_glue_open_file(const char *path) {
    auto file_stream = new RVNGFileStream(path);
    auto input_stream = static_cast<RVNGInputStream *>(file_stream);
    return reinterpret_cast<visio_glue_input_stream *>(input_stream);
}

extern "C" void
visio_glue_close_file(visio_glue_input_stream *stream) {
    if (stream != nullptr) {
        auto input_stream = reinterpret_cast<RVNGInputStream *>(stream);
        delete input_stream;
    }
}

extern "C" bool
visio_glue_document_is_supported(visio_glue_input_stream *stream) {
    auto input_stream = reinterpret_cast<RVNGInputStream *>(stream);
    return VisioDocument::isSupported(input_stream);
}

extern "C" bool
visio_glue_document_parse(visio_glue_input_stream *stream, visio_glue_painter *painter, void *user_ptr) {
    auto input_stream = reinterpret_cast<RVNGInputStream *>(stream);
    CDrawingInterface drawing_interface(painter, user_ptr);
    return VisioDocument::parse(input_stream, &drawing_interface);
}

extern "C" bool
visio_glue_document_parse_stencils(visio_glue_input_stream *stream, visio_glue_painter *painter, void *user_ptr) {
    auto input_stream = reinterpret_cast<RVNGInputStream *>(stream);
    CDrawingInterface drawing_interface(painter, user_ptr);
    return VisioDocument::parseStencils(input_stream, &drawing_interface);
}

extern "C" visio_glue_property_list_iterator *
visio_glue_property_list_iterate(const visio_glue_property_list *list) {
    auto prop_list = reinterpret_cast<const RVNGPropertyList *>(list);
    auto iterator = new RVNGPropertyList::Iter(*prop_list);
    iterator->rewind();
    return reinterpret_cast<visio_glue_property_list_iterator *>(iterator);
}

extern "C" void
visio_glue_property_list_iterator_free(visio_glue_property_list_iterator *iterator) {
    if (iterator != nullptr) {
        auto prop_iterator = reinterpret_cast<RVNGPropertyList::Iter *>(iterator);
        delete prop_iterator;
    }
}

extern "C" bool
visio_glue_property_list_iterator_advance(visio_glue_property_list_iterator *iterator) {
    auto prop_iterator = reinterpret_cast<RVNGPropertyList::Iter *>(iterator);
    return prop_iterator->next();
}

extern "C" const char *
visio_glue_property_list_iterator_key(visio_glue_property_list_iterator *iterator) {
    auto prop_iterator = reinterpret_cast<RVNGPropertyList::Iter *>(iterator);
    return prop_iterator->key();
}

extern "C" visio_glue_property_value
visio_glue_property_list_iterator_value(visio_glue_property_list_iterator *iterator) {
    auto prop_iterator = reinterpret_cast<RVNGPropertyList::Iter *>(iterator);
    const RVNGProperty *prop = (*prop_iterator)();
    if (prop == nullptr) {
        auto value = visio_glue_property_value {
            value: nullptr
        };
        return value;
    }
    RVNGString str = prop->getStr();
    const char *cstr = str.cstr();
    char *dup_cstr = (cstr == nullptr)
        ? nullptr
        : strdup(cstr);
    auto value = visio_glue_property_value {
        value: dup_cstr
    };
    return value;
}

extern "C" void
visio_glue_property_value_free(visio_glue_property_value *value) {
    if (value == nullptr) {
        return;
    }
    if (value->value == nullptr) {
        return;
    }
    free(value->value);
    value->value = nullptr;
}
