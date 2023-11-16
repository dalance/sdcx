# ===================================================================
# TITLE : PERIDOT-NGS / Host bridge sdc
#
#   DEGISN : S.OSAFUNE (J-7SYSTEM WORKS LIMITED)
#   DATE   : 2017/01/23 -> 2017/01/30
#   MODIFY : 2017/04/07
#
# ===================================================================
#
# The MIT License (MIT)
# Copyright (c) 2017,2018 J-7SYSTEM WORKS LIMITED.
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
# of the Software, and to permit persons to whom the Software is furnished to do
# so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
#

set_false_path -from [get_registers {*|peridot_hostbridge:*|altchip_id:*|regout_wire}] -to [get_registers {*|peridot_hostbridge:*|altchip_id:*|dffs[63]}]
set_false_path -to [get_registers {*|peridot_hostbridge:*|peridot_board_i2c:*|scl_in_reg[0]}]
set_false_path -to [get_registers {*|peridot_hostbridge:*|peridot_board_i2c:*|sda_in_reg[0]}]
set_false_path -to [get_registers {*|peridot_hostbridge:*|peridot_config:*|streset_reg[0]}]
set_false_path -to [get_registers {*|peridot_hostbridge:*|peridot_config:*|perireset_reg[0]}]
set_false_path -to [get_registers {*|peridot_hostbridge:*|peridot_config_proc:*|scl_in_reg}]
set_false_path -to [get_registers {*|peridot_hostbridge:*|peridot_config_proc:*|sda_in_reg}]
set_false_path -to [get_registers {*|peridot_hostbridge:*|peridot_config_proc:*|bootsel_reg}]
set_false_path -to [get_registers {*|peridot_hostbridge:*|peridot_config_proc:*|nstatus_reg}]
set_false_path -to [get_registers {*|peridot_hostbridge:*|peridot_config_ru:*|nconfig_in_reg[0]}]
