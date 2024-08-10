SRC := $(shell fd -e adoc .)

DST := $(SRC:.adoc=.html)

ADOC_FLAGS := --safe

.PHONY: all
all: $(DST)

%.html: %.adoc
	asciidoctor ${ADOC_FLAGS} -o $@ $<

.PHONY: clean
clean:
	rm -rf $(DST)
