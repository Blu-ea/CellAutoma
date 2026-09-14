GLSLC?= glslc

SHADER_DIR:=shader

SHADER_SRC_DIR:=src
SHADER_SPV_DIR:=.spv

SRC_DIR:=$(addprefix $(SHADER_DIR)/, $(SHADER_SRC_DIR))
SPV_DIR:=$(addprefix $(SHADER_DIR)/, $(SHADER_SPV_DIR))

SHADER_EXTS:= vert frag geom comp

SOURCES:= $(foreach ext,$(SHADER_EXTS), $(wildcard $(SRC_DIR)/*/shader.$(ext)))

TARGETS := \
$(foreach ext,$(SHADER_EXTS),\
	$(patsubst \
		$(SRC_DIR)/%/shader.$(ext), \
		$(SPV_DIR)/%/$(ext).spv, \
		$(filter $(SRC_DIR)/%/shader.$(ext), $(SOURCES))\
	)\
)

TARGET_SPV_DIR:= $(sort $(dir $(TARGETS)))

all: $(TARGETS)

$(TARGETS): $(SPV_DIR)/%.spv: $(SOURCES) | $(TARGET_SPV_DIR)
	$(GLSLC) $(SRC_DIR)/$(*D)/shader.$(*F) -o $@

$(TARGET_SPV_DIR):
	mkdir -p $@

clean:
	rm -fr $(SPV_DIR)

re: clean all

.PHONY: all clean re
