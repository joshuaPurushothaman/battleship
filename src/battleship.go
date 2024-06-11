package main

type Vec2 struct {
	x, y uint8
}

type Ship struct {
	// unsure if this is the best way to express a Ship...
	size       uint8
	coord      Vec2
	isVertical bool
}

type Cell struct {
	coord Vec2
}

type Board struct {
	ships []Ship
	// cells
}
