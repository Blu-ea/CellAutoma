LST_SRC_SHADER:= shader.frag shader.vert
LST_SPV_SHADER:= $(addsuffix .spv, $(LST_SRC_SHADER:shader.%=%))

SHADER_DIR:=shader

SHADER_SRC_DIR:=src
SHADER_SPV_DIR:=.spv

SHADER_SRC:= $(addprefix $(SHADER_DIR)/$(SHADER_SRC_DIR)/,$(LST_SRC_SHADER))
SHADER_SPV:= $(addprefix $(SHADER_DIR)/$(SHADER_SPV_DIR)/,$(LST_SPV_SHADER))

shader: $(SHADER_SPV)

$(SHADER_DIR)/$(SHADER_SPV_DIR)/%.spv: $(SHADER_DIR)/$(SHADER_SRC_DIR)/shader.% | $(SHADER_DIR)/$(SHADER_SPV_DIR)
	glslc -c $< -o $@


$(SHADER_DIR)/$(SHADER_SPV_DIR) : 
	mkdir -p $@

clean_shader:
	rm -f $(SHADER_SPV)

.PHONY: shader clean_shader
