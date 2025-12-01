// Verilog HDL example - 4-bit counter
module counter (
    input wire clk,
    input wire rst_n,
    input wire enable,
    output reg [3:0] count,
    output wire overflow
);

    assign overflow = (count == 4'hF) & enable;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            count <= 4'b0000;
        end else if (enable) begin
            count <= count + 1'b1;
        end
    end

endmodule

// Testbench
module counter_tb;
    reg clk, rst_n, enable;
    wire [3:0] count;
    wire overflow;

    counter uut (.clk(clk), .rst_n(rst_n), .enable(enable),
                 .count(count), .overflow(overflow));

    initial begin
        clk = 0;
        forever #5 clk = ~clk;
    end
endmodule
