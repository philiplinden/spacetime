SRC ?= learning/notebooks/
DEST ?= docs/notebooks/

define ipynb-to-md
	jupyter nbconvert $(join $(SRC), $(1).ipynb) --to html ;
	mv $(join $(SRC), $(1).html) $(join $(DEST), $(1).md) ;
endef

ipynb2md:
	mkdir -p $(DEST)
	$(foreach nb, \
		$(basename $(notdir $(wildcard $(SRC)*.ipynb))), \
		$(call ipynb-to-md, $(nb)) \
	)
	cp $(SRC)*.png $(DEST)
