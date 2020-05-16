// Code generated by protoc-gen-go. DO NOT EDIT.
// source: service.proto

package rpc

import (
	context "context"
	fmt "fmt"
	proto "github.com/golang/protobuf/proto"
	timestamp "github.com/golang/protobuf/ptypes/timestamp"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	math "math"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

type PriceBase int32

const (
	PriceBase_High  PriceBase = 0
	PriceBase_Low   PriceBase = 1
	PriceBase_Open  PriceBase = 2
	PriceBase_Close PriceBase = 3
)

var PriceBase_name = map[int32]string{
	0: "High",
	1: "Low",
	2: "Open",
	3: "Close",
}

var PriceBase_value = map[string]int32{
	"High":  0,
	"Low":   1,
	"Open":  2,
	"Close": 3,
}

func (x PriceBase) String() string {
	return proto.EnumName(PriceBase_name, int32(x))
}

func (PriceBase) EnumDescriptor() ([]byte, []int) {
	return fileDescriptor_a0b84a42fa06f626, []int{0}
}

type SubscribeRequest struct {
	Symbol               string    `protobuf:"bytes,1,opt,name=symbol,proto3" json:"symbol,omitempty"`
	TickInterval         string    `protobuf:"bytes,2,opt,name=tickInterval,proto3" json:"tickInterval,omitempty"`
	PriceBase            PriceBase `protobuf:"varint,3,opt,name=priceBase,proto3,enum=testbeds.PriceBase" json:"priceBase,omitempty"`
	XXX_NoUnkeyedLiteral struct{}  `json:"-"`
	XXX_unrecognized     []byte    `json:"-"`
	XXX_sizecache        int32     `json:"-"`
}

func (m *SubscribeRequest) Reset()         { *m = SubscribeRequest{} }
func (m *SubscribeRequest) String() string { return proto.CompactTextString(m) }
func (*SubscribeRequest) ProtoMessage()    {}
func (*SubscribeRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_a0b84a42fa06f626, []int{0}
}

func (m *SubscribeRequest) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_SubscribeRequest.Unmarshal(m, b)
}
func (m *SubscribeRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_SubscribeRequest.Marshal(b, m, deterministic)
}
func (m *SubscribeRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_SubscribeRequest.Merge(m, src)
}
func (m *SubscribeRequest) XXX_Size() int {
	return xxx_messageInfo_SubscribeRequest.Size(m)
}
func (m *SubscribeRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_SubscribeRequest.DiscardUnknown(m)
}

var xxx_messageInfo_SubscribeRequest proto.InternalMessageInfo

func (m *SubscribeRequest) GetSymbol() string {
	if m != nil {
		return m.Symbol
	}
	return ""
}

func (m *SubscribeRequest) GetTickInterval() string {
	if m != nil {
		return m.TickInterval
	}
	return ""
}

func (m *SubscribeRequest) GetPriceBase() PriceBase {
	if m != nil {
		return m.PriceBase
	}
	return PriceBase_High
}

type SubscribeResponse struct {
	Symbol               string               `protobuf:"bytes,1,opt,name=symbol,proto3" json:"symbol,omitempty"`
	Timestamp            *timestamp.Timestamp `protobuf:"bytes,2,opt,name=timestamp,proto3" json:"timestamp,omitempty"`
	Price                float64              `protobuf:"fixed64,3,opt,name=price,proto3" json:"price,omitempty"`
	Qty                  float64              `protobuf:"fixed64,4,opt,name=qty,proto3" json:"qty,omitempty"`
	XXX_NoUnkeyedLiteral struct{}             `json:"-"`
	XXX_unrecognized     []byte               `json:"-"`
	XXX_sizecache        int32                `json:"-"`
}

func (m *SubscribeResponse) Reset()         { *m = SubscribeResponse{} }
func (m *SubscribeResponse) String() string { return proto.CompactTextString(m) }
func (*SubscribeResponse) ProtoMessage()    {}
func (*SubscribeResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_a0b84a42fa06f626, []int{1}
}

func (m *SubscribeResponse) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_SubscribeResponse.Unmarshal(m, b)
}
func (m *SubscribeResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_SubscribeResponse.Marshal(b, m, deterministic)
}
func (m *SubscribeResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_SubscribeResponse.Merge(m, src)
}
func (m *SubscribeResponse) XXX_Size() int {
	return xxx_messageInfo_SubscribeResponse.Size(m)
}
func (m *SubscribeResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_SubscribeResponse.DiscardUnknown(m)
}

var xxx_messageInfo_SubscribeResponse proto.InternalMessageInfo

func (m *SubscribeResponse) GetSymbol() string {
	if m != nil {
		return m.Symbol
	}
	return ""
}

func (m *SubscribeResponse) GetTimestamp() *timestamp.Timestamp {
	if m != nil {
		return m.Timestamp
	}
	return nil
}

func (m *SubscribeResponse) GetPrice() float64 {
	if m != nil {
		return m.Price
	}
	return 0
}

func (m *SubscribeResponse) GetQty() float64 {
	if m != nil {
		return m.Qty
	}
	return 0
}

func init() {
	proto.RegisterEnum("testbeds.PriceBase", PriceBase_name, PriceBase_value)
	proto.RegisterType((*SubscribeRequest)(nil), "testbeds.SubscribeRequest")
	proto.RegisterType((*SubscribeResponse)(nil), "testbeds.SubscribeResponse")
}

func init() { proto.RegisterFile("service.proto", fileDescriptor_a0b84a42fa06f626) }

var fileDescriptor_a0b84a42fa06f626 = []byte{
	// 306 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x74, 0x90, 0xcd, 0x4e, 0xc2, 0x40,
	0x14, 0x85, 0x1d, 0xca, 0x5f, 0xaf, 0x3f, 0xa9, 0xa3, 0x31, 0x4d, 0x5d, 0x48, 0x58, 0x11, 0x17,
	0x45, 0x61, 0xe3, 0x1a, 0x13, 0xa3, 0x89, 0x89, 0x5a, 0x59, 0xb9, 0x63, 0xca, 0x15, 0x27, 0x16,
	0x66, 0x98, 0x3b, 0x60, 0x58, 0xfa, 0x00, 0xbe, 0xb3, 0x61, 0x6a, 0xa9, 0x1a, 0xdd, 0xf5, 0x9e,
	0x73, 0x92, 0x7e, 0xdf, 0xc0, 0x2e, 0xa1, 0x59, 0xca, 0x14, 0x63, 0x6d, 0x94, 0x55, 0xbc, 0x69,
	0x91, 0xac, 0xc0, 0x31, 0x45, 0x27, 0x13, 0xa5, 0x26, 0x19, 0x76, 0x5d, 0x2e, 0x16, 0xcf, 0x5d,
	0x2b, 0xa7, 0x48, 0x76, 0x34, 0xd5, 0xf9, 0xb4, 0xfd, 0xce, 0x20, 0x78, 0x5c, 0x08, 0x4a, 0x8d,
	0x14, 0x98, 0xe0, 0x7c, 0x81, 0x64, 0xf9, 0x11, 0xd4, 0x69, 0x35, 0x15, 0x2a, 0x0b, 0x59, 0x8b,
	0x75, 0xfc, 0xe4, 0xeb, 0xe2, 0x6d, 0xd8, 0xb1, 0x32, 0x7d, 0xbd, 0x99, 0x59, 0x34, 0xcb, 0x51,
	0x16, 0x56, 0x5c, 0xfb, 0x23, 0xe3, 0xe7, 0xe0, 0x6b, 0x23, 0x53, 0x1c, 0x8c, 0x08, 0x43, 0xaf,
	0xc5, 0x3a, 0x7b, 0xbd, 0x83, 0xb8, 0xe0, 0x89, 0xef, 0x8b, 0x2a, 0x29, 0x57, 0xed, 0x0f, 0x06,
	0xfb, 0xdf, 0x18, 0x48, 0xab, 0x19, 0xe1, 0xbf, 0x10, 0x17, 0xe0, 0x6f, 0x24, 0x1c, 0xc1, 0x76,
	0x2f, 0x8a, 0x73, 0xcd, 0xb8, 0xd0, 0x8c, 0x87, 0xc5, 0x22, 0x29, 0xc7, 0xfc, 0x10, 0x6a, 0xee,
	0xa7, 0x0e, 0x8b, 0x25, 0xf9, 0xc1, 0x03, 0xf0, 0xe6, 0x76, 0x15, 0x56, 0x5d, 0xb6, 0xfe, 0x3c,
	0xed, 0x83, 0xbf, 0xe1, 0xe4, 0x4d, 0xa8, 0x5e, 0xcb, 0xc9, 0x4b, 0xb0, 0xc5, 0x1b, 0xe0, 0xdd,
	0xaa, 0xb7, 0x80, 0xad, 0xa3, 0x3b, 0x8d, 0xb3, 0xa0, 0xc2, 0x7d, 0xa8, 0x5d, 0x66, 0x8a, 0x30,
	0xf0, 0x7a, 0x0f, 0xd0, 0x18, 0x22, 0xd9, 0x01, 0x8e, 0xf9, 0x15, 0xf8, 0x1b, 0x1d, 0x1e, 0x95,
	0xf2, 0xbf, 0xdf, 0x39, 0x3a, 0xfe, 0xb3, 0xcb, 0xfd, 0xcf, 0xd8, 0xa0, 0xf6, 0xe4, 0x19, 0x9d,
	0x8a, 0xba, 0xb3, 0xea, 0x7f, 0x06, 0x00, 0x00, 0xff, 0xff, 0xbf, 0x16, 0xd7, 0xde, 0xe5, 0x01,
	0x00, 0x00,
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// TestBedClient is the client API for TestBed service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type TestBedClient interface {
	Subscribe(ctx context.Context, in *SubscribeRequest, opts ...grpc.CallOption) (TestBed_SubscribeClient, error)
}

type testBedClient struct {
	cc *grpc.ClientConn
}

func NewTestBedClient(cc *grpc.ClientConn) TestBedClient {
	return &testBedClient{cc}
}

func (c *testBedClient) Subscribe(ctx context.Context, in *SubscribeRequest, opts ...grpc.CallOption) (TestBed_SubscribeClient, error) {
	stream, err := c.cc.NewStream(ctx, &_TestBed_serviceDesc.Streams[0], "/testbeds.TestBed/Subscribe", opts...)
	if err != nil {
		return nil, err
	}
	x := &testBedSubscribeClient{stream}
	if err := x.ClientStream.SendMsg(in); err != nil {
		return nil, err
	}
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	return x, nil
}

type TestBed_SubscribeClient interface {
	Recv() (*SubscribeResponse, error)
	grpc.ClientStream
}

type testBedSubscribeClient struct {
	grpc.ClientStream
}

func (x *testBedSubscribeClient) Recv() (*SubscribeResponse, error) {
	m := new(SubscribeResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// TestBedServer is the server API for TestBed service.
type TestBedServer interface {
	Subscribe(*SubscribeRequest, TestBed_SubscribeServer) error
}

// UnimplementedTestBedServer can be embedded to have forward compatible implementations.
type UnimplementedTestBedServer struct {
}

func (*UnimplementedTestBedServer) Subscribe(req *SubscribeRequest, srv TestBed_SubscribeServer) error {
	return status.Errorf(codes.Unimplemented, "method Subscribe not implemented")
}

func RegisterTestBedServer(s *grpc.Server, srv TestBedServer) {
	s.RegisterService(&_TestBed_serviceDesc, srv)
}

func _TestBed_Subscribe_Handler(srv interface{}, stream grpc.ServerStream) error {
	m := new(SubscribeRequest)
	if err := stream.RecvMsg(m); err != nil {
		return err
	}
	return srv.(TestBedServer).Subscribe(m, &testBedSubscribeServer{stream})
}

type TestBed_SubscribeServer interface {
	Send(*SubscribeResponse) error
	grpc.ServerStream
}

type testBedSubscribeServer struct {
	grpc.ServerStream
}

func (x *testBedSubscribeServer) Send(m *SubscribeResponse) error {
	return x.ServerStream.SendMsg(m)
}

var _TestBed_serviceDesc = grpc.ServiceDesc{
	ServiceName: "testbeds.TestBed",
	HandlerType: (*TestBedServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Subscribe",
			Handler:       _TestBed_Subscribe_Handler,
			ServerStreams: true,
		},
	},
	Metadata: "service.proto",
}
