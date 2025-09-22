package main

import (
	"gopkg.in/yaml.v3"
)

type Foo struct {
	inner yaml.Node
}

func main() {
	m := map[string]string{}

	nested_map := map[string]map[string]string{}
	nested_map["a"] = m
	out, err := yaml.Marshal(nested_map)
	if err != nil {
		panic(err)
	}
	node := yaml.Node{}
	// unmarshal will add yaml document start, cause error when use in sub-yaml document
	err = yaml.Unmarshal(out, &node)
	if err != nil {
		panic(err)
	}
	foo := map[string]yaml.Node{}
	foo["b"] = node

	// works
	other_node := yaml.Node{}
	if err := other_node.Encode(nested_map); err != nil {
		panic(err)
	}

	// error
	out, err = yaml.Marshal(foo)
	if err != nil {
		panic(err)
	}
}
