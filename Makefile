DOCS ?= docs/

define ipynb-to-md
	jupyter nbconvert $(1).ipynb --to html
	cat $(1).html > $(DOCS)/examples/$(1).md
	rm $(1).html
endef

exportnb:
