# ===================================================================
# TITLE : PERIDOT-NGS / WaveTable Sound Genarator
#
#   DEGISN : S.OSAFUNE (J-7SYSTEM WORKS LIMITED)
#   DATE   : 2017/01/23 -> 2017/01/30
#   MODIFY : 2017/06/29
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

set_false_path -to [get_registers {*|peridot_wsg:*|audio_reset_reg}]
set_false_path -to [get_registers {*|peridot_wsg:*|peridot_wsg_businterface:u_busif|done_in_reg}]
set_false_path -to [get_registers {*|peridot_wsg:*|peridot_wsg_businterface:u_busif|extfs_in_reg}]
set_false_path -to [get_registers {*|peridot_wsg:*|peridot_wsg_slotengine:u_slot|start_in_reg}]
set_false_path -from [get_registers {*|peridot_wsg:*|peridot_wsg_businterface:u_busif|keysync_reg}] -to [get_registers {*|peridot_wsg:*|peridot_wsg_slotengine:u_slot|keysync_reg}]
set_false_path -to [get_registers {*|peridot_wsg:*|peridot_wsg_extmodule:u_ext|slot_reset_reg}]
set_false_path -from [get_registers {*|peridot_wsg:*|peridot_wsg_pcm8:*|pcm_speed_reg[*]}] -to [get_registers {*|peridot_wsg:*|peridot_wsg_pcm8:*|pcm_playstep_reg[*]}]
set_false_path -from [get_registers {*|peridot_wsg:*|peridot_wsg_businterface:u_busif|mvol_l_reg[*]}] -to [get_registers {*|peridot_wsg:*|peridot_wsg_audout:u_aud|volume_l_reg[*]}]
set_false_path -from [get_registers {*|peridot_wsg:*|peridot_wsg_businterface:u_busif|mvol_r_reg[*]}] -to [get_registers {*|peridot_wsg:*|peridot_wsg_audout:u_aud|volume_r_reg[*]}]
