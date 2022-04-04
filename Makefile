.PHONY: itext7 test sample-pdf
all: test

test: itext7 target/sample-pdf
	cargo test

# Sample PDF file
target/sample-pdf:
	wget -O target/sample.pdf https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf


# Create the itext7 jarfile
itext7: target/itext7-gradle/build/libs/itext7-gradle-all.jar

# Build the itext7 jarfile
target/itext7-gradle/build/libs/itext7-gradle-all.jar: target/itext7-gradle/build.gradle target/itext7-gradle/settings.gradle
	cd target/itext7-gradle; \
		gradle shadowjar

# build.gradle
define BUILD_GRADLE
plugins {\n
    id 'com.github.johnrengelman.shadow' version '7.1.2'\n
    id 'java'\n
}\n

repositories {\n
    mavenCentral()\n
}\n

dependencies {\n
    implementation 'com.itextpdf:itext7-core:7.2.1'\n
}\n
endef

# Create the build.gradle file
export BUILD_GRADLE
target/itext7-gradle/build.gradle: target/itext7-gradle
	echo $$BUILD_GRADLE > target/itext7-gradle/build.gradle

# settings.gradle
define SETTINGS_GRADLE
rootProject.name = 'itext7-gradle'
endef

# Create the settings.gradle file
export SETTINGS_GRADLE
target/itext7-gradle/settings.gradle: target/itext7-gradle
	echo $$SETTINGS_GRADLE > target/itext7-gradle/settings.gradle

# Create the target/itext7-gradle directory
target/itext7-gradle:
	mkdir -p target/itext7-gradle
