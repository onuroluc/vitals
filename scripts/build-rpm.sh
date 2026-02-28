#!/bin/bash
set -euo pipefail

VERSION="$1"
RPM_ARCH="$2"
TARGET="$3"

mkdir -p rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
cp "target/${TARGET}/release/vitals" rpmbuild/SOURCES/

cat > rpmbuild/SPECS/vitals.spec <<'SPEC'
Name:           vitals
Version:        VERSION_PLACEHOLDER
Release:        1
Summary:        Universal development environment doctor
License:        MIT
URL:            https://github.com/onuroluc/vitals
AutoReqProv:    no

%description
Auto-detects your project stack and diagnoses runtime versions,
dependencies, Docker services, ports, and environment variables.

%install
mkdir -p %{buildroot}/usr/bin
cp %{_sourcedir}/vitals %{buildroot}/usr/bin/vitals

%files
/usr/bin/vitals
SPEC

sed -i "s/VERSION_PLACEHOLDER/${VERSION}/" rpmbuild/SPECS/vitals.spec

rpmbuild \
  --define "_topdir $(pwd)/rpmbuild" \
  --define "_sourcedir $(pwd)/rpmbuild/SOURCES" \
  --target "${RPM_ARCH}" \
  -bb rpmbuild/SPECS/vitals.spec

cp rpmbuild/RPMS/${RPM_ARCH}/*.rpm "vitals-${VERSION}-1.${RPM_ARCH}.rpm"
echo "Built vitals-${VERSION}-1.${RPM_ARCH}.rpm"
