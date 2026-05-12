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
using librevenge::RVNG_SEEK_TYPE;
using librevenge::RVNGString;
using libvisio::VisioDocument;


#define VG_PAINTER_IMPL(method_name, func_name) \
    virtual void method_name() { \
        if (funcs.func_name != nullptr) { \
            (*funcs.func_name)(user_ptr); \
        } \
    }

#define VG_PAINTER_IMPL_PL(method_name, func_name) \
    virtual void method_name(const RVNGPropertyList &propList) { \
        if (funcs.func_name != nullptr) { \
            (*funcs.func_name)(user_ptr, reinterpret_cast<const visio_glue_property_list*>(&propList)); \
        } \
    }

#define VG_PAINTER_IMPL_START_END(start_method, start_func, end_method, end_func) \
    VG_PAINTER_IMPL_PL(start_method, start_func) \
    VG_PAINTER_IMPL(end_method, end_func)


class CDrawingInterface : public RVNGDrawingInterface {
protected:
    visio_glue_painter_funcs funcs;
    void *user_ptr;

public:
    CDrawingInterface(visio_glue_painter_funcs painter_funcs, void *func_user_ptr)
        : funcs(painter_funcs), user_ptr(func_user_ptr) {
    }

    virtual ~CDrawingInterface() {
        if (funcs.destroy == nullptr) {
            return;
        }
        (*funcs.destroy)(user_ptr);
    }

    VG_PAINTER_IMPL_START_END(startDocument, start_document, endDocument, end_document);
    VG_PAINTER_IMPL_PL(setDocumentMetaData, set_document_metadata);
    VG_PAINTER_IMPL_PL(defineEmbeddedFont, define_embedded_font);
    VG_PAINTER_IMPL_START_END(startPage, start_page, endPage, end_page);
    VG_PAINTER_IMPL_START_END(startMasterPage, start_master_page, endMasterPage, end_master_page);
    VG_PAINTER_IMPL_PL(setStyle, set_style);
    VG_PAINTER_IMPL_START_END(startLayer, start_layer, endLayer, end_layer);
    VG_PAINTER_IMPL_START_END(startEmbeddedGraphics, start_embedded_graphics, endEmbeddedGraphics, end_embedded_graphics);
    VG_PAINTER_IMPL_START_END(openGroup, open_group, closeGroup, close_group);
    VG_PAINTER_IMPL_PL(drawRectangle, draw_rectangle);
    VG_PAINTER_IMPL_PL(drawEllipse, draw_ellipse);
    VG_PAINTER_IMPL_PL(drawPolygon, draw_polygon);
    VG_PAINTER_IMPL_PL(drawPolyline, draw_polyline);
    VG_PAINTER_IMPL_PL(drawPath, draw_path);
    VG_PAINTER_IMPL_PL(drawGraphicObject, draw_graphic_object);
    VG_PAINTER_IMPL_PL(drawConnector, draw_connector);
    VG_PAINTER_IMPL_START_END(startTextObject, start_text_object, endTextObject, end_text_object);
    VG_PAINTER_IMPL_START_END(startTableObject, start_table_object, endTableObject, end_table_object);
    VG_PAINTER_IMPL_START_END(openTableRow, open_table_row, closeTableRow, close_table_row);
    VG_PAINTER_IMPL_START_END(openTableCell, open_table_cell, closeTableCell, close_table_cell);
    VG_PAINTER_IMPL_PL(insertCoveredTableCell, insert_covered_table_cell);
    VG_PAINTER_IMPL(insertTab, insert_tab);
    VG_PAINTER_IMPL(insertSpace, insert_space);

    virtual void insertText(const RVNGString &text) {
        if (funcs.insert_text != nullptr) {
            (*funcs.insert_text)(user_ptr, text.cstr(), text.len());
        }
    }

    VG_PAINTER_IMPL(insertLineBreak, insert_line_break);
    VG_PAINTER_IMPL_PL(insertField, insert_field);
    VG_PAINTER_IMPL_START_END(openOrderedListLevel, open_ordered_list_level, closeOrderedListLevel, close_ordered_list_level);
    VG_PAINTER_IMPL_START_END(openUnorderedListLevel, open_unordered_list_level, closeUnorderedListLevel, close_unordered_list_level);
    VG_PAINTER_IMPL_START_END(openListElement, open_list_element, closeListElement, close_list_element);
    VG_PAINTER_IMPL_PL(defineParagraphStyle, define_paragraph_style);
    VG_PAINTER_IMPL_START_END(openParagraph, open_paragraph, closeParagraph, close_paragraph);
    VG_PAINTER_IMPL_PL(defineCharacterStyle, define_character_style);
    VG_PAINTER_IMPL_START_END(openSpan, open_span, closeSpan, close_span);
    VG_PAINTER_IMPL_START_END(openLink, open_link, closeLink, close_link);
};


class CInputStream : public RVNGInputStream {
protected:
    visio_glue_input_stream_funcs funcs;
    void *user_ptr;

public:
    CInputStream(visio_glue_input_stream_funcs stream_funcs, void *stream_user_ptr)
        : funcs(stream_funcs), user_ptr(stream_user_ptr) {
    }

    virtual ~CInputStream() {
        if (funcs.destroy == nullptr) {
            return;
        }
        (*funcs.destroy)(user_ptr);
    }

    virtual bool isStructured() {
        if (funcs.is_structured == nullptr) {
            return false;
        }
        return (*funcs.is_structured)(user_ptr);
    }

    virtual const char *subStreamName(unsigned id) {
        if (funcs.sub_stream_name == nullptr) {
            return nullptr;
        }
        return (*funcs.sub_stream_name)(user_ptr, id);
    }

    virtual unsigned subStreamCount() {
        if (funcs.sub_stream_count == nullptr) {
            return 0;
        }
        return (*funcs.sub_stream_count)(user_ptr);
    }

    virtual bool existsSubStream(const char *name) {
        if (funcs.sub_stream_exists == nullptr) {
            return 0;
        }
        return (*funcs.sub_stream_exists)(user_ptr, name);
    }

    virtual RVNGInputStream *getSubStreamByName(const char *name) {
        if (funcs.sub_stream_by_name == nullptr) {
            return nullptr;
        }
        auto opaque_substream = (*funcs.sub_stream_by_name)(user_ptr, name);
        if (opaque_substream == nullptr) {
            return nullptr;
        }
        auto substream = reinterpret_cast<CInputStream *>(opaque_substream);
        return substream;
    }

    virtual RVNGInputStream *getSubStreamById(unsigned id) {
        if (funcs.sub_stream_by_id == nullptr) {
            return nullptr;
        }
        auto opaque_substream = (*funcs.sub_stream_by_id)(user_ptr, id);
        if (opaque_substream == nullptr) {
            return nullptr;
        }
        auto substream = reinterpret_cast<CInputStream *>(opaque_substream);
        return substream;
    }

    virtual const unsigned char *read(unsigned long numBytes, unsigned long &numBytesRead) {
        if (funcs.read == nullptr) {
            numBytesRead = 0;
            return nullptr;
        }
        return (*funcs.read)(user_ptr, numBytes, &numBytesRead);
    }

    virtual int seek(long offset, RVNG_SEEK_TYPE seekType) {
        if (funcs.seek == nullptr) {
            return -1;
        }
        visio_glue_seek_type c_seek_type;
        switch (seekType) {
            case librevenge::RVNG_SEEK_CUR:
                c_seek_type = VISIO_GLUE_SEEK_CUR;
                break;
            case librevenge::RVNG_SEEK_SET:
                c_seek_type = VISIO_GLUE_SEEK_START;
                break;
            case librevenge::RVNG_SEEK_END:
                c_seek_type = VISIO_GLUE_SEEK_END;
                break;
            default:
                return -1;
        }
        return (*funcs.seek)(user_ptr, offset, c_seek_type);
    }

    virtual long tell() {
        if (funcs.tell == nullptr) {
            return -1;
        }
        return (*funcs.tell)(user_ptr);
    }

    virtual bool isEnd() {
        if (funcs.is_end == nullptr) {
            return false;
        }
        return (*funcs.is_end)(user_ptr);
    }
};

extern "C" visio_glue_input_stream *
visio_glue_new_input_stream(visio_glue_input_stream_funcs funcs, void *user_ptr) {
    auto input_stream = new CInputStream(funcs, user_ptr);
    return reinterpret_cast<visio_glue_input_stream *>(input_stream);
}

extern "C" visio_glue_painter *
visio_glue_new_painter(visio_glue_painter_funcs funcs, void *user_ptr) {
    auto painter = new CDrawingInterface(funcs, user_ptr);
    return reinterpret_cast<visio_glue_painter *>(painter);
}

extern "C" void
visio_glue_input_stream_free(visio_glue_input_stream *stream) {
    if (stream == nullptr) {
        return;
    }
    auto input_stream = reinterpret_cast<CInputStream *>(stream);
    delete input_stream;
}

extern "C" void
visio_glue_painter_free(visio_glue_painter *painter) {
    if (painter == nullptr) {
        return;
    }
    auto drawing_painter = reinterpret_cast<CDrawingInterface *>(painter);
    delete drawing_painter;
}

extern "C" bool
visio_glue_document_is_supported(visio_glue_input_stream *stream) {
    auto input_stream = reinterpret_cast<CInputStream *>(stream);
    return VisioDocument::isSupported(input_stream);
}

extern "C" bool
visio_glue_document_parse(visio_glue_input_stream *stream, visio_glue_painter *painter) {
    auto input_stream = reinterpret_cast<CInputStream *>(stream);
    auto drawing_painter = reinterpret_cast<CDrawingInterface *>(painter);
    return VisioDocument::parse(input_stream, drawing_painter);
}

extern "C" bool
visio_glue_document_parse_stencils(visio_glue_input_stream *stream, visio_glue_painter *painter) {
    auto input_stream = reinterpret_cast<CInputStream *>(stream);
    auto drawing_painter = reinterpret_cast<CDrawingInterface *>(painter);
    return VisioDocument::parseStencils(input_stream, drawing_painter);
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
            nullptr
        };
        return value;
    }
    RVNGString str = prop->getStr();
    const char *cstr = str.cstr();
    char *dup_cstr = (cstr == nullptr)
        ? nullptr
        : strdup(cstr);
    auto value = visio_glue_property_value {
        dup_cstr
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
