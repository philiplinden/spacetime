NOTEBOOKS ?= docs/notebooks/
DOCS ?= docs/examples/

define ipynb-to-md
	jupyter nbconvert $(join $(NOTEBOOKS), $(1).ipynb) --to html ;
	mv $(join $(NOTEBOOKS), $(1).html) $(join $(DOCS), $(1).md) ;
endef

ipynb2md:
	mkdir -p $(DOCS)
	$(foreach nb, \
		$(basename $(notdir $(wildcard notebooks/*.ipynb))), \
		$(call ipynb-to-md, $(nb)) \
	)
