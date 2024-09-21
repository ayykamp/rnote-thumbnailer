#
# spec file for package raw-thumbnailer
#
# Copyright (c) 2024 Hubert Figuière
# This file and all modifications and additions to the pristine
# package are under the same license as the package itself.
#

# norootforbuild

Summary: Camera raw thumbnailing for GNOME
Name: raw-thumbnailer
Version: 47.0.1
Release: 1
License: GNU General Public License (GPL)
Group: System/GUI/GNOME
%define prefix   /usr
Source: ./%{name}-%{version}.tar.xz
BuildRequires: cargo-rpm-macros >= 24
BuildRequires: meson
BuildRequires: shared-mime-info

%description
Camera raw thumbnailer for GNOME. Works by extracting the thumbnail
from the file if it is possible.

%prep
%autosetup -p1
%cargo_prep -v vendor

%build
%meson
%meson_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
%meson_install

%post
usr/bin/update-mime-database /usr/share/mime >/dev/null

%postun
usr/bin/update-mime-database /usr/share/mime >/dev/null

%files
%defattr(-,root,root)
%doc README NEWS COPYING ChangeLog

%{prefix}/bin/raw-thumbnailer
%{_datadir}/thumbnailers/raw.thumbnailer
%{_datadir}/mime/packages/raw-thumbnailer.xml

%changelog

