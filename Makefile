NOTEBOOKS ?= learning/notebooks/
PYTHON_DOCS ?= docs/python/notebooks/

define ipynb-to-md
	jupyter nbconvert $(join $(NOTEBOOKS), $(1).ipynb) --to html ;
	mv $(join $(NOTEBOOKS), $(1).html) $(join $(PYTHON_DOCS), $(1).md) ;
endef

ipynb2md:
	mkdir -p $(PYTHON_DOCS)
	$(foreach nb, \
		$(basename $(notdir $(wildcard $(NOTEBOOKS)*.ipynb))), \
		$(call ipynb-to-md, $(nb)) \
	)
