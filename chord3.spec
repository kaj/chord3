Name:    chord3
Version: 0.3.0
Release: 1%{?dist}
Summary: Create PDF songbooks from chopro source
URL:     https://github.com/kaj/chord3
License: Beerware

Source0: https://github.com/kaj/%{name}/archive/v%{version}.tar.gz#/%{name}-%{version}.tar.gz

BuildRequires: cargo

%description
Chord3 takes a (set of) chopro file(s) and converts them to a single
PDF file.  If no file names are given as arguments, a single chopro
files is read from standard input.  Chopro files is simply text files
with chord names in brackets and some other options in braces, on
separate lines.

%prep
%autosetup

%build
cargo build --release

%install
mkdir -p %{buildroot}/%{_bindir}
install -p -m 755 target/release/chord3 %{buildroot}/%{_bindir}

%files
%{_bindir}/chord3

%changelog
* Sun Oct 16 2016 Rasmus Kaj <rasmus@krats.se> - 0.3.0-1
- Initial packaging.
