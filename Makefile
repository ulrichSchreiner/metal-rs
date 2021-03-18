.PHONY:
generate:
	openapi-cli -i ./metal-api.json -g rust -o . --skip-validate-spec --type-mappings=integer=i64 --additional-properties=packageName=metal-rs,packageVersion=0.0.1