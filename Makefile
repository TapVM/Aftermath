default:
	@echo "Hey! Do \`make help\` to know about all the build tasks, have fun!"

classbasket:
	@echo "\e[36mBuilding random Java snippets for JVM development...\e[0m\n"
	cd class_basket; javac *.java;
	@chmod +x ./scripts/shell.sh
	@cd scripts; ./shell.sh;

help:
	@echo "Welcome to the Aftermath build system 👋"
	@echo "The following are all the available make tasks.\n"

	@echo "\`buildjvm\` -> Builds the Aftermath JVM."
	@echo "\`buildjdk\` -> Builds OpenJDK with Aftermath."
	@echo "\`classbasket\` -> Builds random Java snippets for JVM development."
	@echo "\`test\` -> Tests the JVM."
	@echo "\`advancedtest\` -> Advanced testing for the JVM. (Requires an internet connection)"
	@echo "\`bench\` -> Benchmarks performance, optionally compared to Hotspot and GraalVM."
	@echo "\`clean\` -> Cleans the workspace.\n"

	@echo "That's it for now, we hope you enjoy your stay =)"
