# AMPL file for solving a system of linear equalities

# Define variables
var x;  # variable x
var y;  # variable y
var z;  # variable z
var dx; # variable dx
var dy; # variable dy
var dz; # variable dz
var t;  # variable t
var s;  # variable s
#var u;  # variable u
var v;  # variable v

# Objective function (dummy objective since we are only solving equalities)
minimize ObjectiveFunction: 0;

# Constraints
subject to Eq1:
    x + t*dx = 288998070705911 - 63*t;

subject to Eq2:
    y + t*dy = 281498310692304 + 25*t;

subject to Eq3:
    z + t*dz = 225433163951734 + 66*t;

subject to Eq4:
    x + s*dx = 267942038821112 + 97*s;

subject to Eq5:
    y + s*dy = 319206560980050 - 61*s;

subject to Eq6:
    z + s*dz = 228821793591214 + 70*s;

#subject to Eq7:
#    x + u*dx = 444805588706877 - 167*u;
#
#subject to Eq8:
#    y + u*dy = 248504563833176 + 337*u;
#
#subject to Eq9:
#    z + u*dz = 237588696365934 + 94*u;

subject to Eq10:
    x + v*dx = 394123042863727 + 55*v;

subject to Eq11:
    y + v*dy = 301207059897078 - 100*v;

subject to Eq12:
    z + v*dz = 223821779603118 + 150*v;
