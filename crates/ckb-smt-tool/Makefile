MOLC := moleculec
MOLC_VERSION := 0.7.5

.PHONY: generate-protocols
generate-protocols: check-moleculec-version src/types/generated.rs

.PHONY: check-moleculec-version
check-moleculec-version:
	test "$$(${MOLC} --version | awk '{ print $$2  }' | tr -d ' ')" = ${MOLC_VERSION}

src/types/generated.rs: schemas/types.mol
	${MOLC} --language rust --schema-file $< | rustfmt > $@
