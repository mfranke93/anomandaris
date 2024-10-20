EXTRA_TARGETS := print_date print_hello

.PHONY: $(EXTRA_TARGETS)

all: $(EXTRA_TARGETS)

print_date:
	date

print_hello:
	echo "Hello world!"