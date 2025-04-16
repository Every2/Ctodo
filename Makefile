BIN_DIR = bin

TARGET = $(BIN_DIR)/out

SRC = todo.c

CFLAGS = $(shell pkg-config --cflags gtk4)
LIBS = $(shell pkg-config --libs gtk4)

$(TARGET): $(SRC) $(BIN_DIR)
	gcc $(CFLAGS) -o $(TARGET) $(SRC) $(LIBS)

$(BIN_DIR):
	mkdir $(BIN_DIR)

run: $(BIN_DIR)/out
	./$(BIN_DIR)/out

clean:
	rm -f $(BIN_DIR)
