EXTRA_TARGETS := print_date print_hello

#

# Test comments
# test


# .PHONY marks these targets as "always re-build"
.PHONY: $(EXTRA_TARGETS)

# "all" is the default target
all: $(EXTRA_TARGETS)

print_date: print_hello
	date

print_hello:
	echo -n "Hello "
	echo "world!"