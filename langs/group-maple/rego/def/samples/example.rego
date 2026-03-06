# Rego policy example
# Demonstrates basic Rego language features for OPA (Open Policy Agent)

package example.authz

import rego.v1
import data.users
import input.request as req

# Default deny all requests
default allow := false

# Allow admin users
allow if {
    some user in data.users
    user.name == input.user
    user.role == "admin"
}

# Allow users to access their own resources
allow if {
    input.user == input.resource.owner
}

# Check if user has required permission
has_permission(user, permission) if {
    some role in user.roles
    some p in data.role_permissions[role]
    p == permission
}

# Generate a set of violated rules
violations contains msg if {
    some resource in input.resources
    not resource.compliant
    msg := sprintf("Resource %v is not compliant", [resource.name])
}

# Object rule with key-value pairs
resource_labels[name] := value if {
    some resource in input.resources
    name := resource.name
    value := resource.labels
}

# Partial set rule
public_endpoints contains endpoint if {
    some endpoint in input.endpoints
    endpoint.visibility == "public"
}

# Arithmetic and comparison
max_replicas := 10
min_replicas := 2

valid_replica_count if {
    input.replicas >= min_replicas
    input.replicas <= max_replicas
}

# Array comprehension
admin_names := [name |
    some user in data.users
    user.role == "admin"
    name := user.name
]

# Object comprehension
user_emails := {name: email |
    some user in data.users
    name := user.name
    email := user.email
}

# Set comprehension
unique_roles := {role |
    some user in data.users
    some role in user.roles
}

# Every keyword (universal quantification)
all_resources_tagged if {
    every resource in input.resources {
        resource.tags != null
        count(resource.tags) > 0
    }
}

# With modifier for testing
test_allow_admin if {
    allow with input as {"user": "alice", "resource": {"owner": "bob"}}
        with data.users as [{"name": "alice", "role": "admin"}]
}
