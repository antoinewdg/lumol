# Molecular dynamics of water

In this example, we will simulate a molecular liquid of high scientific
interest: water. At the same time we will learn how we can reuse a potential
definition between multiple simulations, and how we can switch the potential
definition in a simulation.

You will need three input files for this simulation:

- [water.xyz]: the initial configuration;
- [water.toml]: the simulation input;
- [water-fSCP.toml]: the potentials definitions.

The simulation run in a few minutes (up to 10), and will write the trajectory of
the system.

[water.xyz]: data/water.xyz
[water.toml]: data/water.toml
[water-fSCP.toml]: data/water-fSCP.toml

# The input files commented

## `water.toml` file

The main input file is pretty similar to the previous examples, with two
novelties:

- The `guess_bonds = true` entry tell Lumol to try to guess the bonds from the
  distances in the XYZ file. This is needed because we want to simulate a
  molecule, but there is not bonding information inside the XYZ format. If we
  used a PDB file instead, this would not be needed;
- The `potentials = "water-fSCP.toml"` tell Lumol to read the potentials from
  the `water-fSCP.toml` file. Using this type of input for the potentials allow
  to share the same potential for multiple simulations, and to easily change
  the potential used in the simulation.

```toml
[input]
version = 1

[[systems]]
file = "water.xyz"
cell = 28.0
guess_bonds = true
potentials = "water-fSCP.toml"

[[simulations]]
nsteps = 5000
outputs = [
    {type = "Trajectory", file = "trajectory.xyz", frequency = 10}
]

[simulations.propagator]
type = "MolecularDynamics"
timestep = "1 fs"
```

## `water-fSCP.toml` file

`water-fSCP.toml` is a standalone potential input file. It contains the same
data as a potential definition inside a `[[systems]]` section, but without the
`system.potential` prefix on all sections names.

In this input, we start by defining which version of the input format we are
using.

```toml
# f-SPC model of water, using Ewald summation for electrostatics
[input]
version = 1
```

Then, we can define some global input data: the pair potential cutoff and the
atomic charges.

```toml
[global]
cutoff = "14 A"

[charges]
O = -0.82
H = 0.41
```


The pair potential section contains the usual pairs declarations, with a few
additional options.

```toml
[[pairs]]
atoms = ["O", "O"]
lj = {sigma = "3.16 A", epsilon = "0.155 kcal/mol"}
```

We can add a `restriction` to restrict a specific pair interaction to some kind
of pairs. Here we will only account for H-H pairs inside the same molecule.

```toml
[[pairs]]
atoms = ["H", "H"]
harmonic = {k = "79.8 kcal/mol/A^2", x0 = "1.633 A"}
restriction = "IntraMolecular"
```

We can also define a non-interacting pair interaction! This is useful when some
atoms does not interact in the model we use.

```toml
[[pairs]]
atoms = ["H", "O"]
null = {}
```

Then comes the bonds and angles definitions. These are the interactions used
between bonded particles (or angles formed by two bonds and dihedral angles
formed by three bonds).

```toml
[[bonds]]
atoms = ["O", "H"]
harmonic = {k = "1054.2 kcal/mol/A^2", x0 = "1.0 A"}

[[angles]]
atoms = ["H", "O", "H"]
harmonic = {k = "75.9 kcal/mol/rad^2", x0 = "109.5 deg"}
```

Finally we specify how to compute the electrostatic interaction, this time using
the Ewald sum. We can restrict the coulombic interactions to only apply between
particles not in the same molecule using a `restriction` here too.

```toml
[coulomb]
ewald = {cutoff = "8.5 A", kmax = 3}
restriction = "InterMolecular"
```
