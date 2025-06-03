import numpy as np


periodic_table = atomic_symbols = (
    'X',  # placeholder
    'H', 'He',
    'Li', 'Be', 'B',  'C',  'N', 'O',  'F',  'Ne',
    'Na', 'Mg', 'Al', 'Si', 'P', 'S',  'Cl', 'Ar',
    'K',  'Ca', 'Sc', 'Ti', 'V', 'Cr', 'Mn', 'Fe', 'Co', 'Ni', 'Cu', 'Zn', 'Ga', 'Ge', 'As', 'Se', 'Br', 'Kr',
    'Rb', 'Sr', 'Y', 'Zr', 'Nb', 'Mo', 'Tc', 'Ru', 'Rh', 'Pd', 'Ag', 'Cd', 'In', 'Sn', 'Sb', 'Te', 'I',  'Xe',
    'Cs', 'Ba',
    'La', 'Ce', 'Pr', 'Nd', 'Pm', 'Sm', 'Eu', 'Gd', 'Tb', 'Dy', 'Ho', 'Er', 'Tm', 'Yb', 'Lu',
    'Hf', 'Ta', 'W',  'Re', 'Os', 'Ir', 'Pt', 'Au', 'Hg', 'Tl', 'Pb', 'Bi', 'Po', 'At', 'Rn',
    'Fr', 'Ra',
    'Ac','Th', 'Pa', 'U', 'Np', 'Pu', 'Am', 'Cm', 'Bk', 'Cf', 'Es', 'Fm', 'Md', 'No', 'Lr',
    'Rf', 'Db', 'Sg', 'Bh', 'Hs', 'Mt', 'Ds', 'Rg', 'Cn', 'Nh', 'Fl', 'Mc', 'Lv', 'Ts', 'Og',
)  # fmt: skip

atomic_numbers = {symbol: Z for Z, symbol in enumerate(periodic_table)}
atomic_names = (
    'X',
    'Hydrogen',
    'Helium',
    'Lithium',
    'Beryllium',
    'Boron',
    'Carbon',
    'Nitrogen',
    'Oxygen',
    'Fluorine',
    'Neon',
    'Sodium',
    'Magnesium',
    'Aluminum',
    'Silicon',
    'Phosphorus',
    'Sulfur',
    'Chlorine',
    'Argon',
    'Potassium',
    'Calcium',
    'Scandium',
    'Titanium',
    'Vanadium',
    'Chromium',
    'Manganese',
    'Iron',
    'Cobalt',
    'Nickel',
    'Copper',
    'Zinc',
    'Gallium',
    'Germanium',
    'Arsenic',
    'Selenium',
    'Bromine',
    'Krypton',
    'Rubidium',
    'Strontium',
    'Yttrium',
    'Zirconium',
    'Niobium',
    'Molybdenum',
    'Technetium',
    'Ruthenium',
    'Rhodium',
    'Palladium',
    'Silver',
    'Cadmium',
    'Indium',
    'Tin',
    'Antimony',
    'Tellurium',
    'Iodine',
    'Xenon',
    'Cesium',
    'Barium',
    'Lanthanum',
    'Cerium',
    'Praseodymium',
    'Neodymium',
    'Promethium',
    'Samarium',
    'Europium',
    'Gadolinium',
    'Terbium',
    'Dysprosium',
    'Holmium',
    'Erbium',
    'Thulium',
    'Ytterbium',
    'Lutetium',
    'Hafnium',
    'Tantalum',
    'Tungsten',
    'Rhenium',
    'Osmium',
    'Iridium',
    'Platinum',
    'Gold',
    'Mercury',
    'Thallium',
    'Lead',
    'Bismuth',
    'Polonium',
    'Astatine',
    'Radon',
    'Francium',
    'Radium',
    'Actinium',
    'Thorium',
    'Protactinium',
    'Uranium',
    'Neptunium',
    'Plutonium',
    'Americium',
    'Curium',
    'Berkelium',
    'Californium',
    'Einsteinium',
    'Fermium',
    'Mendelevium',
    'Nobelium',
    'Lawrencium',
    'Rutherfordium',
    'Dubnium',
    'Seaborgium',
    'Bohrium',
    'Hassium',
    'Meitnerium',
    'Darmstadtium',
    'Roentgenium',
    'Copernicium',
    'Nihonium',
    'Flerovium',
    'Moscovium',
    'Livermorium',
    'Tennessine',
    'Oganesson',
)
# Covalent radii from:
#
#  Covalent radii revisited,
#  Beatriz Cordero, Verónica Gómez, Ana E. Platero-Prats, Marc Revés,
#  Jorge Echeverría, Eduard Cremades, Flavia Barragán and Santiago Alvarez,
#  Dalton Trans., 2008, 2832-2838 DOI:10.1039/B801115J
UNKN = 0.2
covalent_radius = (
    # X, placeholder
    UNKN,
    # H    He
    0.31, 0.28,
    # Li    Be     B     C     N     O     F    Ne
    1.28, 0.96, 0.84, 0.76, 0.71, 0.66, 0.57, 0.58,
    # Na    Mg    Al    Si     P     S    Cl    Ar
    1.66, 1.41, 1.21, 1.11, 1.07, 1.05, 1.02, 1.06,
    #  K    Ca    Sc    Ti     V    Cr    Mn    Fe    Co    Ni    Cu    Zn    Ga    Ge    As    Se    Br    Kr
    2.03, 1.76, 1.70, 1.60, 1.53, 1.39, 1.39, 1.32, 1.26, 1.24, 1.32, 1.22, 1.22, 1.20, 1.19, 1.20, 1.20, 1.16,
    # Rb    Sr     Y    Zr    Nb    Mo    Tc    Ru    Rh    Pd    Au    Cd    In    Sn    Sb    Te     I    Xe
    2.20, 1.95, 1.90, 1.75, 1.64, 1.54, 1.47, 1.46, 1.42, 1.39, 1.45, 1.44, 1.42, 1.39, 1.39, 1.38, 1.39, 1.40,
    # Cs    Ba
    2.44, 2.15,
    # La    Ce    Pr    Nd    Pm    Sm    Eu    Gd    Tb    Dy    Ho    Er    Tm    Yb    Lu
    2.07, 2.04, 2.03, 2.01, 1.99, 1.98, 1.98, 1.96, 1.94, 1.92, 1.92, 1.89, 1.90, 1.87, 1.87,
    # Hf    Ta     W    Re    Os    Ir    Pt    Au    Hg    Tl    Pb    Bi    Po    At    Rn
    1.75, 1.70, 1.62, 1.51, 1.44, 1.41, 1.36, 1.36, 1.32, 1.45, 1.46, 1.48, 1.40, 1.50, 1.50,
    # Fr    Ra
    2.60, 2.21,
    # Ac    Th    Pa     U    Np    Pu    Am    Cm    Bk    Cf    Es    Fm    Md    No    Lr
    2.15, 2.06, 2.00, 1.96, 1.90, 1.87, 1.80, 1.69, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN,
    # Rf    Db    Sg    Bh    Hs    Mt    Ds    Rg    Cn    Nh    Fl    Mc    Lv    Ts    Og
    UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN,
)  # fmt: skip

vdw_radius = (
    # X, placeholder
    UNKN,
    # H    He
    1.2,  1.40,
    # Li    Be     B     C     N     O     F    Ne
    1.82, 1.53, 1.92, 1.70, 1.55, 1.52, 1.47, 1.54,
    # Na    Mg    Al    Si     P     S    Cl    Ar
    2.27, 1.73, 1.84, 2.10, 1.80, 1.80, 1.75, 1.88,
    #  K    Ca    Sc    Ti     V    Cr    Mn    Fe    Co    Ni    Cu    Zn    Ga    Ge    As    Se    Br    Kr
    2.75, 2.31, 2.11, 2.00, 2.00, 2.00, 2.00, 2.00, 2.00, 1.63, 1.40, 1.39, 1.87, 2.11, 1.85, 1.90, 1.85, 2.02,
    # Rb    Sr     Y    Zr    Nb    Mo    Tc    Ru    Rh    Pd    Au    Cd    In    Sn    Sb    Te     I    Xe
    3.03, 2.49, 2.00, 2.00, 2.00, 2.00, 2.00, 2.00, 1.63, 1.72, 1.58, 1.93, 2.17, 2.06, 2.06, 2.06, 1.98, 2.16,
    # Cs    Ba
    3.43, 2.68,
    # La    Ce    Pr    Nd    Pm    Sm    Eu    Gd    Tb    Dy    Ho    Er    Tm    Yb    Lu
    2.10, 2.10, 2.10, 2.10, 2.10, 2.10, 2.10, 2.10, 2.10, 2.10, 2.10, 2.10, 2.10, 2.10, 2.10,
    # Hf    Ta     W    Re    Os    Ir    Pt    Au    Hg    Tl    Pb    Bi    Po    At    Rn
    2.10, 2.10, 2.10, 2.10, 2.10, 2.10, 1.75, 1.66, 1.55, 1.96, 2.02, 2.07, 1.97, 2.02, 2.20,
    # Fr    Ra
    3.48, 2.83,
    # Ac    Th    Pa     U    Np    Pu    Am    Cm    Bk    Cf    Es    Fm    Md    No    Lr
    2.20, 2.20, 2.20, 2.20, 2.20, 2.20, 2.20, 2.20, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN,
    # Rf    Db    Sg    Bh    Hs    Mt    Ds    Rg    Cn    Nh    Fl    Mc    Lv    Ts    Og
    UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN,
)  # fmt: skip

metallic_radius = (
    # X, placeholder
    UNKN,
    # H    He
    UNKN, UNKN,
    # Li    Be     B     C     N     O     F    Ne
    1.52, 1.12, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN,
    # Na    Mg    Al    Si     P     S    Cl    Ar
    1.86, 1.60, 1.43, UNKN, UNKN, UNKN, UNKN, UNKN,
    #  K    Ca    Sc    Ti     V    Cr    Mn    Fe    Co    Ni    Cu    Zn    Ga    Ge    As    Se    Br    Kr
    2.27, 1.97, 1.62, 1.47, 1.34, 1.28, 1.27, 1.26, 1.25, 1.24, 1.28, 1.34, 1.35, UNKN, UNKN, UNKN, UNKN, UNKN,
    # Rb    Sr     Y    Zr    Nb    Mo    Tc    Ru    Rh    Pd    Au    Cd    In    Sn    Sb    Te     I    Xe
    2.48, 2.15, 1.80, 1.60, 1.46, 1.39, 1.36, 1.34, 1.34, 1.37, 1.44, 1.51, 1.67, UNKN, UNKN, UNKN, UNKN, UNKN,
    # Cs    Ba
    2.65, 2.22,
    # La     Ce     Pr     Nd     Pm     Sm     Eu     Gd     Tb     Dy     Ho     Er     Tm    Yb     Lu
    1.87, 1.818, 1.824, 1.814, 1.834, 1.804, 1.804, 1.804, 1.773, 1.781, 1.762, 1.761, 1.759, 1.76, 1.738,
    # Hf    Ta     W    Re    Os     Ir     Pt    Au    Hg    Tl    Pb    Bi    Po    At    Rn
    1.59, 1.46, 1.39, 1.37, 1.35, 1.355, 1.385, 1.44, 1.51, 1.70, UNKN, UNKN, UNKN, UNKN, UNKN,
    # Fr    Ra
    UNKN, UNKN,
    # Ac    Th    Pa     U    Np    Pu    Am    Cm    Bk    Cf    Es    Fm    Md    No    Lr
    UNKN, 1.79, 1.63, 1.56, 1.55, 1.59, 1.73, 1.74, 1.70, 1.86, 1.86, UNKN, UNKN, UNKN, UNKN,
    # Rf    Db    Sg    Bh    Hs    Mt    Ds    Rg    Cn    Nh    Fl    Mc    Lv    Ts    Og
    UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN, UNKN,
)  # fmt: skip

# Atomic masses are based on:
#
#   Meija, J., Coplen, T., Berglund, M., et al. (2016). Atomic weights of
#   the elements 2013 (IUPAC Technical Report). Pure and Applied Chemistry,
#   88(3), pp. 265-291. Retrieved 30 Nov. 2016,
#   from doi:10.1515/pac-2015-0305
#
# Weights of [H, He, B, N, O, Mg, Si, S, Cl, Br, Tl] are taken from Table 3 for
# *conventional atomic weights* with range in Table 1 in comment. Those with stable
# isotope are taken from Table 1 for *standard atomic weights 2013* with uncertainties
# ignored. Others with no stable isotopes are taken from Table 4.
atomic_masses = atomic_masses_iupac2016 = (
    1.0,  # X, placeholder
    1.008,  # H [1.00784, 1.00811]
    4.002602,  # He
    6.94,  # Li [6.938, 6.997]
    9.0121831,  # Be
    10.81,  # B [10.806, 10.821]
    12.011,  # C [12.0096, 12.0116]
    14.007,  # N [14.00643, 14.00728]
    15.999,  # O [15.99903, 15.99977]
    18.998403163,  # F
    20.1797,  # Ne
    22.98976928,  # Na
    24.305,  # Mg [24.304, 24.307]
    26.9815385,  # Al
    28.085,  # Si [28.084, 28.086]
    30.973761998,  # P
    32.06,  # S [32.059, 32.076]
    35.45,  # Cl [35.446, 35.457]
    39.948,  # Ar
    39.0983,  # K
    40.078,  # Ca
    44.955908,  # Sc
    47.867,  # Ti
    50.9415,  # V
    51.9961,  # Cr
    54.938044,  # Mn
    55.845,  # Fe
    58.933194,  # Co
    58.6934,  # Ni
    63.546,  # Cu
    65.38,  # Zn
    69.723,  # Ga
    72.630,  # Ge
    74.921595,  # As
    78.971,  # Se
    79.904,  # Br [79.901, 79.907]
    83.798,  # Kr
    85.4678,  # Rb
    87.62,  # Sr
    88.90584,  # Y
    91.224,  # Zr
    92.90637,  # Nb
    95.95,  # Mo
    97.90721,  # 98Tc
    101.07,  # Ru
    102.90550,  # Rh
    106.42,  # Pd
    107.8682,  # Ag
    112.414,  # Cd
    114.818,  # In
    118.710,  # Sn
    121.760,  # Sb
    127.60,  # Te
    126.90447,  # I
    131.293,  # Xe
    132.90545196,  # Cs
    137.327,  # Ba
    138.90547,  # La
    140.116,  # Ce
    140.90766,  # Pr
    144.242,  # Nd
    144.91276,  # 145Pm
    150.36,  # Sm
    151.964,  # Eu
    157.25,  # Gd
    158.92535,  # Tb
    162.500,  # Dy
    164.93033,  # Ho
    167.259,  # Er
    168.93422,  # Tm
    173.054,  # Yb
    174.9668,  # Lu
    178.49,  # Hf
    180.94788,  # Ta
    183.84,  # W
    186.207,  # Re
    190.23,  # Os
    192.217,  # Ir
    195.084,  # Pt
    196.966569,  # Au
    200.592,  # Hg
    204.38,  # Tl [204.382, 204.385]
    207.2,  # Pb
    208.98040,  # Bi
    208.98243,  # 209Po
    209.98715,  # 210At
    222.01758,  # 222Rn
    223.01974,  # 223Fr
    226.02541,  # 226Ra
    227.02775,  # 227Ac
    232.0377,  # Th
    231.03588,  # Pa
    238.02891,  # U
    237.04817,  # 237Np
    244.06421,  # 244Pu
    243.06138,  # 243Am
    247.07035,  # 247Cm
    247.07031,  # 247Bk
    251.07959,  # 251Cf
    252.0830,  # 252Es
    257.09511,  # 257Fm
    258.09843,  # 258Md
    259.1010,  # 259No
    262.110,  # 262Lr
    267.122,  # 267Rf
    268.126,  # 268Db
    271.134,  # 271Sg
    270.133,  # 270Bh
    269.1338,  # 269Hs
    278.156,  # 278Mt
    281.165,  # 281Ds
    281.166,  # 281Rg
    285.177,  # 285Cn
    286.182,  # 286Nh
    289.190,  # 289Fl
    289.194,  # 289Mc
    293.204,  # 293Lv
    293.208,  # 293Ts
    294.214,  # 294Og
)


# list with atomic number z, short name, full name, valence,
# valence electrons, covalent radius, vdW radius, metallic radius


def get_covalent_volume(elements: str | list[str], scale=1) -> float | list[float]:
    """Get atom/structure volume according to covalent radii

    Parameters
    ----------
    elements : str | list[str]
        Name of one element or a list of elements.
    scale : float, default 1
        ...

    Returns
    -------
    volume : float | list[float]
        Covalent radii volume of one element or list of each of them.
    """

    if isinstance(elements, list):
        return [
            (4 * np.pi * covalent_radius[atomic_numbers[element]] ** 3) * scale / 3
            for element in elements
        ]
    elif isinstance(elements, str):
        return (4 * np.pi * covalent_radius[atomic_numbers[elements]] ** 3) * scale / 3
    else:
        raise ValueError(
            f"Can not calculate volume according to your element name {elements} or elements list {elements}"
        )

def period(number: int):
    """The periodic table row of the element.
    Note: For lanthanoids and actinoids, the row is always 6 or 7,
    respectively.
    """
    size_of_each_row = (2, 8, 8, 18, 18, 32, 32)
    total = 0
    if number == 0:
        return 0
    if 57 <= number <= 71:
        return 6
    if 89 <= number <= 103:
        return 7
    for idx, size in enumerate(size_of_each_row, start=1):
        total += size
        if total >= number:
            return idx
    return 8

def group(z: int):
    """The periodic table group of the element.
    Note: For lanthanoids and actinoids, the group is always 3.
    """
    if z == 0:
        return 0
    if z == 1:
        return 1
    if z == 2:
        return 18

    if 3 <= z <= 18:
        if (z - 2) % 8 == 0:
            return 18
        if (z - 2) % 8 <= 2:
            return (z - 2) % 8
        return 10 + (z - 2) % 8

    if 19 <= z <= 54:
        if (z - 18) % 18 == 0:
            return 18
        return (z - 18) % 18

    if (57 <= z <= 71) or (89 <= z <= 103):
        return 3

    if (z - 54) % 32 == 0:
        return 18
    if (z - 54) % 32 >= 18:
        return (z - 54) % 32 - 14
    return (z - 54) % 32

def generate_element_info_json() -> None:
    result = []
    print(f"atomic_symbols: {len(atomic_symbols)}")
    print(f"atomic_names: {len(atomic_names)}")
    print(f"atomic_masses: {len(atomic_masses)}")

    print(f"vdw_radius: {len(vdw_radius)}")
    print(f"covalent_radius: {len(covalent_radius)}")
    print(f"metallic_radius: {len(metallic_radius)}")

    for idx, sym in enumerate(atomic_symbols):
        element = {
            "symbol": sym,
            "name": atomic_names[idx],
            "atomic_number": idx,
            "atomic_mass": atomic_masses[idx],
            "period": period(idx),
            "group": group(idx),
            "van_der_Waals_radius": vdw_radius[idx],
            "covalent_radius": covalent_radius[idx],
            "metallic_radius": metallic_radius[idx],
        }
        result.append(element)
    import json
    s = json.dumps(result, indent=2)
    print(s, file=open('element_info.json', 'w'))


if __name__ == '__main__':
    generate_element_info_json()