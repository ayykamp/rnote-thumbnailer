#
# spec file for package rnote-thumbnailer
#
# Copyright (c) 2024 Hubert Figuière
# This file and all modifications and additions to the pristine
# package are under the same license as the package itself.
#

# norootforbuild

Summary: Rnote thumbnailing for GNOME
Name: rnote-thumbnailer
Version: 0.1.0
Release: 1
License: GNU General Public License (GPL)
Group: System/GUI/GNOME
%define prefix   /usr
Source: ./%{name}-%{version}.tar.xz
BuildRequires: cargo-rpm-macros >= 24
BuildRequires: meson
BuildRequires: shared-mime-info

%description
Rnote thumbnailer.

%global debug_package %{nil}

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

%{prefix}/bin/rnote-thumbnailer
%{_datadir}/thumbnailers/rnote.thumbnailer
%{_datadir}/mime/packages/com.github.flxzt.rnote.xml

%changelog
